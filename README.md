# 漂亮的无边框时钟
<img width="259" height="258" alt="image" src="https://github.com/user-attachments/assets/74b771e5-e995-495c-a63a-32d2b840e929" />
https://github.com/yangliangguang/time/releases
一个使用 Rust 和 Slint 构建的现代化无边框桌面时钟应用。

## 特性

- 🎨 **现代化设计**: 渐变色彩和圆形设计
- 🚫 **无边框窗口**: 透明背景，融入桌面
- 📅 **完整时间显示**: 显示时间、日期和星期
- 🎯 **置顶显示**: 始终保持在其他窗口之上
- 🖱️ **可拖拽**: 点击拖拽移动窗口位置
- ✨ **视觉效果**: 阴影、发光和装饰元素

## 界面预览

时钟采用深蓝色渐变背景，配以蓝色发光边框和装饰点，显示：
- 当前时间（大字体，单色字体）
- 当前日期（中文格式）
- 星期几（中文显示）

## 技术栈

- **Rust**: 系统编程语言，保证性能和安全性
- **Slint**: 现代化的 Rust GUI 框架
- **Chrono**: 时间和日期处理库
- **Tokio**: 异步运行时，用于定时器

## 构建和运行

### 前置要求

- Rust 1.70 或更高版本
- Cargo 包管理器

### 安装依赖

```bash
# 克隆或下载项目后，在项目目录中运行：
cargo build
```

### 运行应用

```bash
cargo run
```

### 构建发布版本

```bash
cargo build --release
```

## 使用说明

1. **启动应用**: 运行 `cargo run` 启动时钟
2. **移动窗口**: 点击时钟任意位置并拖拽来移动窗口
3. **关闭应用**: 点击右上角的 "×" 按钮关闭

## 项目结构

```
time/
├── Cargo.toml          # 项目配置和依赖
├── build.rs            # 构建脚本
├── src/
│   └── main.rs         # 主要 Rust 代码
├── ui/
│   └── clock.slint     # UI 界面定义
└── README.md           # 项目说明
```

## 自定义

### 修改颜色主题

编辑 `ui/clock.slint` 文件中的颜色值：
- `background`: 主背景渐变
- `border-color`: 边框颜色
- `color`: 文字颜色

### 调整窗口大小

在 `clock.slint` 中修改：
```slint
width: 200px;   // 窗口宽度
height: 200px;  // 窗口高度
```

### 更改字体

修改时间显示的字体：
```slint
font-family: "Consolas", "Monaco", monospace;
```

## 许可证

本项目采用 MIT 许可证。
