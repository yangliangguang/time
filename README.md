# 漂亮的无边框时钟 (Beautiful Clock)

一个使用 Rust 和 Slint 构建的现代化无边框桌面时钟应用，具有苹果风格的毛玻璃效果设计和实时天气显示功能。

## ✨ 核心特性

- 🎨 **苹果风格设计**: 淡绿色毛玻璃效果，现代化圆形界面
- 🚫 **无边框窗口**: 透明背景，完美融入桌面环境
- ⏰ **双重时钟显示**: 数字时间显示 + 模拟时钟表盘
- 📅 **完整时间信息**: 时间、日期、星期几（中文显示）
- 🌤️ **实时天气**: 自动获取宁海地区天气信息，每小时更新
- 🎯 **始终置顶**: 保持在所有窗口之上
- 🖱️ **可拖拽移动**: 点击任意位置拖拽移动窗口
- 📱 **系统托盘**: 后台运行，托盘图标管理
- 🚀 **开机自启**: 自动添加到 Windows 启动项
- 💻 **托盘双击切换**: 双击托盘图标隐藏/显示窗口
- 🔄 **加载动画**: 启动时显示优雅的加载界面

## 🖼️ 界面预览

### 主界面
- **外观**: 408×408 像素圆形窗口，淡绿色毛玻璃效果
- **时钟**: 中央显示模拟时钟，带有时针、分针、秒针和数字刻度
- **数字显示**: 时间（44px 大字体）、日期、星期几
- **天气信息**: 实时显示天气状况和温度
- **加载界面**: 启动时显示 2 秒加载动画

### 交互功能
- **拖拽**: 点击任意位置拖拽移动窗口
- **双击**: 双击窗口最小化到系统托盘；双击托盘图标隐藏/显示窗口
- **托盘菜单**: 右键托盘图标显示菜单（显示/退出）
- **托盘点击**: 左键点击托盘图标显示窗口

## 🛠️ 技术栈

- **Rust 2021**: 系统编程语言，保证性能和内存安全
- **Slint 1.12**: 现代化跨平台 GUI 框架
- **Chrono 0.4**: 时间和日期处理
- **Tokio 1.0**: 异步运行时和定时器
- **Reqwest 0.11**: HTTP 客户端，获取天气数据
- **Tray-icon 0.13**: 系统托盘功能
- **Windows API**: Windows 特定功能（启动项、窗口管理）

## 🚀 快速开始

### 前置要求

- **Rust**: 1.70 或更高版本
- **操作系统**: Windows (已测试)
- **网络连接**: 用于获取天气信息

### 安装和运行

1. **克隆项目**
```bash
git clone <项目地址>
cd time
```

2. **构建项目**
```bash
cargo build
```

3. **运行应用**
```bash
cargo run
```

4. **构建发布版本**
```bash
cargo build --release
```

### 🎯 快速配置

应用首次运行时会：
- 自动添加到 Windows 启动项
- 窗口居中显示在屏幕中央
- 开始获取天气信息
- 创建系统托盘图标

## 📁 项目结构

```
time/
├── .cargo/
│   └── config.toml         # Cargo 配置
├── resources/              # 资源文件
│   ├── README.md          # 图标使用说明
│   ├── clock-icon.ico     # 应用图标
│   ├── clock-icon.svg     # SVG 图标
│   ├── hour_hand.svg      # 时针图像
│   ├── minute_hand.svg    # 分针图像
│   └── second_hand.svg    # 秒针图像
├── src/
│   ├── bin/               # 辅助工具
│   │   ├── check_ico.rs   # 图标检查工具
│   │   └── check_ico_detailed.rs
│   ├── lunar.rs           # 农历计算（已移除）
│   └── main.rs            # 主程序
├── ui/
│   ├── clock.slint        # 主时钟界面
│   └── simple_menu.slint  # 简单菜单界面
├── build.rs               # 构建脚本
├── Cargo.toml             # 项目配置
└── README.md              # 项目文档
```

## ⚙️ 功能详解

### 天气功能
- **数据源**: 中国天气网 API (宁海地区)
- **更新频率**: 每小时自动更新
- **显示内容**: 天气类型、温度范围、风向风力
- **错误处理**: 网络异常时显示默认信息

### 时钟功能
- **数字时钟**: 24 小时制，秒级精度
- **模拟时钟**: 带有时针、分针、秒针的传统表盘
- **动态刻度**: 根据上午/下午显示不同的小时数字
- **实时更新**: 每秒更新一次

### 系统集成
- **Windows 启动项**: 自动添加/移除启动项
- **任务栏隐藏**: 使用 WS_EX_TOOLWINDOW 样式隐藏任务栏图标
- **系统托盘**: 完整的托盘交互支持

## 🎨 自定义配置

### 修改颜色主题

编辑 `ui/clock.slint` 文件：

```slint
// 主背景色
background: rgba(144, 238, 144, 0.8); // 淡绿色

// 边框颜色
border-color: rgba(255, 255, 255, 0.7);

// 文字颜色
color: rgba(28, 28, 30, 0.9); // 苹果黑色
```

### 调整窗口大小

```slint
width: 408px;   // 窗口宽度
height: 408px;  // 窗口高度
border-radius: 408px; // 保持圆形
```

### 修改字体样式

```slint
// 时间显示
font-size: 44px;
font-weight: 300;
letter-spacing: 3px;

// 日期显示
font-size: 19px;
font-weight: 300;
```

### 天气数据源

修改 `src/main.rs` 中的天气 API：

```rust
// 更改为其他城市（需要获取对应的城市代码）
let url = format!("http://t.weather.itboy.net/api/weather/city/{}", city_code);
```

## 🔧 开发指南

### 编译选项

项目使用优化的发布配置：

```toml
[profile.release]
opt-level = "s"      # 优化大小
strip = true         # 删除调试信息
lto = true          # 链接时优化
```

### 依赖管理

主要依赖及版本：
- `slint = "1.12"` - GUI 框架
- `chrono = "0.4"` - 时间处理
- `tokio = "1"` - 异步运行时
- `reqwest = "0.11"` - HTTP 客户端
- `tray-icon = "0.13.5"` - 系统托盘

### 构建脚本

`build.rs` 负责：
- 编译 Slint UI 文件
- 设置 Windows 应用图标
- 处理资源文件

## 🐛 故障排除

### 常见问题

1. **编译错误**: 确保 Rust 版本 ≥ 1.70
2. **图标不显示**: 检查 `resources/` 文件夹中的图标文件
3. **天气不显示**: 检查网络连接和防火墙设置
4. **启动项失败**: 以管理员权限运行一次

### 调试模式

启用调试输出：
```bash
RUST_LOG=debug cargo run
```

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

### 开发环境设置

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 📞 支持

如有问题或建议，请：
- 提交 [GitHub Issue](../../issues)
- 查看 [项目 Wiki](../../wiki)
- 联系开发者

---

**享受这个漂亮的桌面时钟吧！** ⏰✨