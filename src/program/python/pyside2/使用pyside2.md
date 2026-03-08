# PySide2 笔记

## 说明

- 本页记录 `PySide2` 常见 GUI 组件的最小使用方式.
- 当前重点是多标签页和多页面切换相关控件.

## 常见控件

- `QTabWidget`: 适合直接展示标签页切换.
- `QStackedWidget`: 适合把页面切换和导航组件拆开管理.
- `QTabBar`: 适合需要自定义标签行为时单独使用.

## 实现多标签页

参考:

- <https://blog.csdn.net/wjh_init/article/details/78881066>

选择建议:

- 页面结构比较标准时, 优先用 `QTabWidget`.
- 需要自己控制侧边栏, 顶部菜单或向导流程时, 优先用 `QStackedWidget`.

## 最小示例

```python
from PySide2.QtWidgets import QApplication, QLabel, QTabWidget

app = QApplication([])
widget = QTabWidget()
widget.addTab(QLabel("第一页"), "Tab 1")
widget.addTab(QLabel("第二页"), "Tab 2")
widget.resize(400, 240)
widget.show()
app.exec_()
```

## 使用建议

- 若页面较多, 建议把每个页面封装成独立 `QWidget` 类.
- 若标签内容需要动态刷新, 尽量把状态更新逻辑和页面切换逻辑分开.
- 若后续继续整理, 可补充信号槽, 菜单栏, 对话框和线程协作相关示例.
