# 资源文件夹

请将您的ico图标文件放在此文件夹中，文件名应为`icon.ico`。

**重要提示**：请确保使用有效的.ico格式文件，而不是文本文件。

## 如何创建有效的ico文件：

### 方法1：使用在线转换工具
1. 访问在线图标转换网站（如 https://convertio.co/png-ico/ 或 https://online-converter.com/png_to_ico）
2. 上传您喜欢的图片或使用以下示例时钟图片：
   ![示例时钟图片](clock-icon.svg)
3. 将图片转换为ico格式
4. 下载并重命名为`icon.ico`

### 方法2：使用图像编辑软件
1. 使用GIMP、Photoshop或其他图像编辑软件打开图片
2. 导出或另存为ico格式
3. 将文件重命名为`icon.ico`

### 方法3：使用专门的图标制作工具
1. 下载并安装专门的图标编辑器（如IcoFX、Axialis IconWorkshop等）
2. 创建新图标或导入现有图片
3. 保存为ico格式并重命名为`icon.ico`

## 项目配置说明：

项目已经配置好支持图标资源，您只需要：

1. 将有效的ico文件放入此文件夹并命名为`icon.ico`
2. 重新编译项目：`cargo build`

编译成功后，生成的exe文件将带有您指定的图标。