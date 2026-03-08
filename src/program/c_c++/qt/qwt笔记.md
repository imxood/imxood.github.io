# Qwt

## 说明

- `Qwt` 是 Qt 生态中常见的绘图与仪表盘扩展库.
- 适合做曲线图, 仪表盘, 标尺和工程类数据可视化控件.

## 适用场景

- 需要在 Qt 桌面应用中展示实时曲线或工程图表.
- 需要比原生 Qt Widget 更偏工程化的仪表盘和标尺控件.
- 需要在 Qt Designer 中直接拖拽使用第三方图形控件.

## 参考

- [Qwt User's Guide](https://qwt.sourceforge.io/qwtinstall.html)

## 安装 Qwt

```sh
svn checkout https://svn.code.sf.net/p/qwt/code/trunk qwt-code
cd qwt-code/qwt
qmake
make -j10
sudo make install
```

## 添加到 Qt Designer

```sh
cp /usr/local/qwt-6.3.0-svn/plugins/designer/libqwt_designer_plugin.so Qt5.14.0/Tools/QtCreator/lib/Qt/plugins/designer
```

## 常见注意点

- 不同 Qt 版本的插件目录可能不同, 使用前需先确认本地 Qt Creator 安装路径.
- 若插件无法显示在 Designer 中, 先检查 `Qt` 版本, 插件 ABI 和安装目录是否匹配.
- 若只是做简单图表, 也可以先评估是否使用 `Qt Charts` 或直接绘制自定义控件.
