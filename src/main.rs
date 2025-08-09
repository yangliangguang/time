#![windows_subsystem = "windows"]

use slint::*;
use std::time::Duration;
use chrono::{Local, Datelike, Weekday, Timelike};
use reqwest;
use tokio;
use tokio::time::{interval, Duration as TokioDuration};
#[cfg(target_os = "windows")]
use std::ffi::OsStr;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
#[cfg(target_os = "windows")]
use std::fmt;

// System tray imports
use tao::event_loop::{EventLoopBuilder, ControlFlow};
use tray_icon::{menu::{Menu, MenuItem, MenuEvent}, TrayIconBuilder, Icon, TrayIconEvent};
use anyhow::Result;

// Windows-specific imports
#[cfg(target_os = "windows")]
use i_slint_backend_winit::winit::raw_window_handle::RawWindowHandle;
#[cfg(target_os = "windows")]
use slint::ComponentHandle;  // 导入ComponentHandle trait
#[cfg(target_os = "windows")]
use windows::{
    core::{HRESULT, HSTRING},
    Win32::{
        Foundation::{ERROR_FILE_NOT_FOUND, HWND, S_OK, WIN32_ERROR},
        System::Registry::{RegSetValueExW, RegDeleteValueW, RegCloseKey, HKEY, HKEY_CURRENT_USER, KEY_WRITE, REG_SZ, RegCreateKeyExW},
        UI::WindowsAndMessaging::{SetWindowLongPtrW, GWL_EXSTYLE, WS_EX_TOOLWINDOW},
    },
};
#[cfg(target_os = "windows")]
use windows_core::PCWSTR;

#[cfg(target_os = "windows")]
fn get_hwnd(window: &Window) -> Option<HWND> {
    use i_slint_backend_winit::WinitWindowAccessor;
    window.with_winit_window(|winit_window| {
        use i_slint_backend_winit::winit::raw_window_handle::HasWindowHandle;
        match winit_window.window_handle() {
            Ok(handle) => {
                match handle.as_raw() {
                    RawWindowHandle::Win32(h) => Some(HWND(isize::from(h.hwnd))),
                    _ => None,
                }
            },
            Err(_) => None,
        }
    }).flatten()
}

slint::include_modules!();

/// 自定义错误类型
#[cfg(target_os = "windows")]
#[derive(Debug)]
pub struct StartupError(String);

#[cfg(target_os = "windows")]
impl fmt::Display for StartupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(target_os = "windows")]
impl std::error::Error for StartupError {}

#[cfg(target_os = "windows")]
impl From<slint::SharedString> for StartupError {
    fn from(s: slint::SharedString) -> Self {
        StartupError(s.to_string())
    }
}

// 为 StartupError 实现 From<windows_core::Error> trait
#[cfg(target_os = "windows")]
impl From<windows_core::Error> for StartupError {
    fn from(e: windows_core::Error) -> Self {
        StartupError(e.to_string())
    }
}



fn weekday_to_chinese(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Mon => "星期一",
        Weekday::Tue => "星期二",
        Weekday::Wed => "星期三",
        Weekday::Thu => "星期四",
        Weekday::Fri => "星期五",
        Weekday::Sat => "星期六",
        Weekday::Sun => "星期日",
    }
}

/// 将字符串转换为宽字符串（UTF-16）
#[cfg(target_os = "windows")]
fn to_wide_string(s: &str) -> Vec<u16> {
    OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))  // 添加 null 终止符
        .collect()
}

/// 添加程序到 Windows 启动项
#[cfg(target_os = "windows")]
fn add_to_startup() -> Result<(), StartupError> {
    unsafe {
        let mut hkey = HKEY::default();
        let subkey = to_wide_string("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
        let app_name = to_wide_string("BeautifulClock");
        
        // 创建或打开注册表项
        let result = RegCreateKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            0,
            None,
            windows::Win32::System::Registry::REG_OPEN_CREATE_OPTIONS(0),
            KEY_WRITE,
            None,
            &mut hkey,
            Some(std::ptr::null_mut()),
        );
        
        if let Err(e) = result {
            return Err(StartupError(e.to_string()));
        }
        
        // 获取当前可执行文件路径
        let exe_path = std::env::current_exe().map_err(|e| StartupError(e.to_string()))?;
        let exe_path_str = exe_path.to_string_lossy().to_string();
        let exe_path_wide = to_wide_string(&exe_path_str);
        // 将宽字符串转换为字节序列
        let exe_path_bytes: Vec<u8> = exe_path_wide.iter().flat_map(|&w| w.to_le_bytes()).collect();

        // 设置注册表值
        let result = RegSetValueExW(
            hkey,
            PCWSTR(app_name.as_ptr()),
            0,
            REG_SZ,
            Some(&exe_path_bytes),
        );
        
        // 关闭注册表键
        RegCloseKey(hkey);
        
        if let Err(e) = result {
            return Err(StartupError(e.to_string()));
        }
        
        Ok(())
    }
}

/// 从 Windows 启动项中移除程序
#[cfg(target_os = "windows")]
fn remove_from_startup() -> Result<(), StartupError> {
    unsafe {
        let mut hkey = HKEY::default();
        let subkey = to_wide_string("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
        let app_name = to_wide_string("BeautifulClock");
        
        // 创建或打开注册表项
        let result = RegCreateKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            0,
            None,
            windows::Win32::System::Registry::REG_OPEN_CREATE_OPTIONS(0),
            KEY_WRITE,
            None,
            &mut hkey,
            Some(std::ptr::null_mut()),
        );
        
        if let Err(e) = result {
            return Err(StartupError(e.to_string()));
        }
        
        // 删除注册表值
        let result = RegDeleteValueW(hkey, PCWSTR(app_name.as_ptr()));
        
        // 关闭注册表键
        RegCloseKey(hkey);
        
        if let Err(e) = result {
            // 2 表示文件未找到，可以忽略
            if e.code().0 != 2 {
                return Err(StartupError(e.to_string()));
            }
        }
        
        Ok(())
    }
}

async fn fetch_weather() -> Result<String, Box<dyn std::error::Error>> {
    // 使用中国天气网的API获取宁海天气信息
    // 宁海的天气预报URL: https://e.weather.com.cn/mweather/101210408.shtml
    // 这里我们使用一个模拟的API端点来获取天气数据
    // 添加随机数参数以确保获取最新数据
    let random_param = rand::random::<u32>();
    let url = std::format!("http://t.weather.itboy.net/api/weather/city/101210408?_t={}", random_param).to_string();
    
    let response = reqwest::get(url).await?;
    let weather_data: serde_json::Value = response.json().await?;
    
    // 解析天气数据
    if let Some(weather_info) = weather_data.get("data") {
        let weather = weather_info.get("forecast").and_then(|f| f.get(0)).ok_or("Failed to get forecast")?;
        let type_str = weather.get("type").and_then(|v| v.as_str()).unwrap_or("未知");
        let high_temp = weather.get("high").and_then(|v| v.as_str()).unwrap_or("");
        let low_temp = weather.get("low").and_then(|v| v.as_str()).unwrap_or("");
        let fx = weather.get("fx").and_then(|v| v.as_str()).unwrap_or("未知");
        let fl = weather.get("fl").and_then(|v| v.as_str()).unwrap_or("");
        
        // 格式化温度信息
        let temp_str = if !high_temp.is_empty() && !low_temp.is_empty() {
            std::format!("{}℃~{}℃", 
                low_temp.trim_start_matches("低温").trim_end_matches("℃"),
                high_temp.trim_start_matches("高温").trim_end_matches("℃"))
        } else {
            "温度未知".to_string()
        };
        
        Ok(std::format!("{} {} \n{}{}", type_str, temp_str, fx, fl))
    } else {
        Err("Failed to get weather data".into())
    }
}

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    // 添加程序到启动项
    #[cfg(target_os = "windows")]
    {
        if let Err(e) = add_to_startup() {
            eprintln!("无法添加到启动项: {}", e.to_string());
        }
    }
    
    // 创建系统托盘事件循环
    let event_loop = EventLoopBuilder::new().build();
    let menu = Menu::new();
    menu.append(&MenuItem::new("显示", true, None)).map_err(|e| slint::PlatformError::from(e.to_string()))?;
    menu.append(&MenuItem::new("退出", true, None)).map_err(|e| slint::PlatformError::from(e.to_string()))?;
    
    // 创建系统托盘图标
    let icon_path = std::path::Path::new("resources/clock-icon.ico");
    let icon = if icon_path.exists() {
        Icon::from_path(icon_path, None)
            .unwrap_or(Icon::from_rgba(vec![0,0,0,0], 1, 1).unwrap())
    } else {
        Icon::from_resource_name("icon.ico", None)
            .unwrap_or(Icon::from_resource(1, None)
            .unwrap_or(Icon::from_rgba(vec![0,0,0,0], 1, 1).unwrap()))
    };
    
    let _tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("漂亮时钟")
            .with_icon(icon)
            .build()
            .map_err(|e| slint::PlatformError::from(e.to_string()))?,
    );
    
    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();
    
    let ui = Clock::new()?;
    println!("应用启动");
    
    // 获取天气信息并设置到UI
    match fetch_weather().await {
        Ok(weather) => ui.set_current_weather(weather.into()),
        Err(e) => eprintln!("获取天气信息失败: {}", e),
    }
    
    // 设置窗口初始位置到屏幕中心
    // 注意：在实际应用中，可能需要使用特定于平台的 API 来获取屏幕尺寸
    // 这里我们使用一个固定的屏幕尺寸作为示例 (1920x1080)
    let screen_width = 1920.0;
    let screen_height = 1080.0;
    let window_size = ui.window().size();
    let x = (screen_width - window_size.width as f32) / 2.0;
    let y = (screen_height - window_size.height as f32) / 2.0;
    ui.window().set_position(slint::LogicalPosition::new(x, y));
    
    // 在Windows上设置窗口样式以隐藏任务栏图标
    #[cfg(target_os = "windows")]
    {
        // 注意：需要确保在窗口显示之后再设置样式
        if let Some(hwnd) = get_hwnd(&ui.window()) {
            unsafe {
                SetWindowLongPtrW(hwnd, GWL_EXSTYLE, WS_EX_TOOLWINDOW.0 as isize);
                // 可能需要调用SetWindowPos来刷新窗口样式
                // use windows::Win32::UI::WindowsAndMessaging::{SetWindowPos, SWP_NOMOVE, SWP_NOSIZE};
                // SetWindowPos(hwnd, None, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);
            }
        }
    }
    
    // 初始化时间显示
    let now = Local::now();
    ui.set_current_time(now.format("%H:%M:%S").to_string().into());
    ui.set_current_date(now.format("%Y年%m月%d日").to_string().into());
    ui.set_current_day(weekday_to_chinese(now.weekday()).into());
    
    // 处理窗口拖拽
    let ui_weak = ui.as_weak();
    ui.on_window_moved(move |delta_x: f32, delta_y: f32| {
        if let Some(ui) = ui_weak.upgrade() {
            let window = ui.window();
            let current_pos = window.position(); // current_pos.x and current_pos.y are i32
            let new_x = current_pos.x as f32 + delta_x;
            let new_y = current_pos.y as f32 + delta_y;
            window.set_position(slint::LogicalPosition::new(new_x, new_y));
        }
    });

    let ui_handle_exit = ui.as_weak();
    ui.on_request_exit(move || {
        if let Some(ui) = ui_handle_exit.upgrade() {
            println!("退出应用");
            ui.window().hide().unwrap(); // 隐藏窗口而不是完全退出应用
        }
    });
    
    // 添加设置回调处理
    let ui_handle_settings = ui.as_weak();
    ui.on_show_settings(move || {
        if let Some(_ui) = ui_handle_settings.upgrade() {
            // 这里可以实现设置功能，暂时先打印信息
            println!("显示设置菜单");
            // 例如，可以显示一个设置窗口或弹出对话框
        }
    });
    
    // 克隆 UI 句柄用于定时器
    let ui_handle = ui.as_weak();
    let ui_handle_weather = ui.as_weak();
    
    // 用于跟踪上次更新的时间
    let mut last_time = String::new();
    let mut last_date = String::new();
    let mut last_day = String::new();
    
    // 启动定时器更新时间
    let timer = slint::Timer::default();
    timer.start(slint::TimerMode::Repeated, Duration::from_millis(1000), move || {
        let ui = match ui_handle.upgrade() {
            Some(ui) => ui,
            None => return,
        };        
        let now = Local::now();
        
        // 格式化时间
        let time_str = now.format("%H:%M:%S").to_string();
        let date_str = now.format("%Y年%m月%d日").to_string();
        let day_str = weekday_to_chinese(now.weekday());
        
        // 计算时针、分针和秒针的角度
        let seconds = now.second() as f32;
        let minutes = now.minute() as f32 + seconds / 60.0;
        let hours = (now.hour() % 12) as f32 + minutes / 60.0;
        
        let second_angle = seconds * 6.0; // 360度/60秒 = 6度/秒
        let minute_angle = minutes * 6.0; // 360度/60分钟 = 6度/分钟
        let hour_angle = hours * 30.0;    // 360度/12小时 = 30度/小时
        
        // 只有当时间确实发生变化时才更新UI
        if time_str != last_time || date_str != last_date || day_str != last_day {
            last_time = time_str.clone();
            last_date = date_str.clone();
            last_day = day_str.to_string();
            
            // 更新 UI
            ui.set_current_time(time_str.into());
            ui.set_current_date(date_str.into());
            ui.set_current_day(day_str.into());
            
            // 更新时钟指针角度
            ui.set_hour_angle(hour_angle);
            ui.set_minute_angle(minute_angle);
            ui.set_second_angle(second_angle);
        }
    });
    
    // 启动定时器每小时更新天气
    tokio::spawn(async move {
        let mut interval = interval(TokioDuration::from_secs(60 * 60)); // 1小时
        loop {
            interval.tick().await;
            match fetch_weather().await {
                Ok(weather) => {
                    let ui_handle = ui_handle_weather.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(ui) = ui_handle.upgrade() {
                            ui.set_current_weather(weather.into());
                        }
                    });
                },
                Err(e) => eprintln!("获取天气信息失败: {}", e),
            }
        }
    });

    // 模拟加载过程，2秒后显示主界面
    let ui_loading_handle = ui.as_weak();
    slint::Timer::single_shot(Duration::from_secs(2), move || {
        if let Some(ui) = ui_loading_handle.upgrade() {
            ui.set_loading(false);
        }
    });
    
    // 克隆 UI 句柄用于系统托盘事件处理
    let ui_handle_tray = ui.as_weak();
    
    // 在单独的任务中轮询系统托盘事件
    tokio::spawn(async move {
        loop {
            // 处理菜单事件
            if let Ok(event) = menu_channel.try_recv() {
                match event.id().0.as_str() {
                    "显示" => {
                        // 使用 upgrade_in_event_loop 来避免 Send trait 问题
                        let _ = ui_handle_tray.upgrade_in_event_loop(|ui| {
                            ui.window().show().unwrap();
                        });
                    },
                    "退出" => {
                        // 使用 upgrade_in_event_loop 来避免 Send trait 问题
                        let _ = ui_handle_tray.upgrade_in_event_loop(|ui| {
                            ui.window().hide().unwrap();
                        });
                    },
                    _ => {}
                }
            }
            
            // 处理托盘图标事件
            if let Ok(event) = tray_channel.try_recv() {
                // 匹配左键点击事件
                if event.click_type == tray_icon::ClickType::Left {
                    // 使用 upgrade_in_event_loop 来避免 Send trait 问题
                    let _ = ui_handle_tray.upgrade_in_event_loop(|ui| {
                        ui.window().show().unwrap();
                    });
                }
            }
            
            // 短暂休眠以避免过度占用 CPU
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
    });
    
    // 运行应用
    ui.run()?;
    
    Ok(())
}