# Tesseract OCR

## 说明

- 本页记录 `Tesseract OCR` 的构建入口和接入时需要关注的依赖.
- 适合在 Windows 或本地原生项目中回忆“从源码到可用”的最小路线.

## 参考资料

- 官方编译文档: <https://tesseract-ocr.github.io/tessdoc/Compiling.html#windows>

## 构建关注点

- `Tesseract` 通常还依赖 `Leptonica`.
- Windows 下经常需要同时确认 `CMake`, 编译器工具链和第三方库路径.
- 若使用包管理器, 可优先评估 `vcpkg` 这类更省事的方式.

## 常见接入流程

1. 先准备 `Tesseract` 和 `Leptonica` 所需依赖.
2. 确认 `CMake` 或 IDE 工程可以找到头文件和库文件.
3. 完成最小命令行识别验证.
4. 再接入自己的桌面程序或后端服务.

## Windows 侧排查思路

- 编译期失败: 先检查编译器版本和依赖库路径.
- 运行期失败: 再检查动态库是否可被正确加载.
- 识别效果差: 再回到训练数据, 输入图像质量和语言包配置.

## 后续可补主题

- `vcpkg` 安装记录.
- `CMake` 示例工程.
- 语言数据文件放置路径.
- 与 GUI 或批处理程序的集成示例.
