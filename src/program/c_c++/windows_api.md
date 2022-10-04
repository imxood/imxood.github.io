## windows 桌面开发 (官方教程)

https://learn.microsoft.com/zh-cn/windows/win32/learnwin32/learn-to-program-for-windows

## windows 消息循环 的详细讲解

https://learn.microsoft.com/zh-cn/windows/win32/learnwin32/window-messages

基本的过程是:
- 操作系统在消息队列上放置 WM_LBUTTONDOWN 消息。
- 程序调用 GetMessage 函数。
- GetMessage 从队列中拉取 WM_LBUTTONDOWN 消息，并填充 MSG 结构。
- 程序调用 TranslateMessage 和 DispatchMessage 函数。
- 在 DispatchMessage 中，操作系统调用窗口过程。
- 窗口过程可以响应消息或忽略它。

## SendMessage / PostMessage

参见: https://learn.microsoft.com/zh-cn/windows/win32/api/winuser/nf-winuser-sendmessage
https://blog.csdn.net/mcw_720624/article/details/117192221

SendMessage发出消息后，会等待回应结果，而PostMessage发出消息后，不等待回应

SendMessage(hWnd, Msg, wParam, lParam)

``` py
import win32gui, win32con

hwnd = win32gui.FindWindow('Chrome_WidgetWin_1', 'CSDN - 专业开发者社区 - Google Chrome')
win32gui.SetForegroundWindow(hwnd) # 激活窗口至前端(这行语句不能少)

win32gui.SendMessage(hwnd, win32con.WM_KEYDOWN, win32con.VK_NEXT, 0)
```

若向记事本等字处理窗口发送消息时，不要用整个父窗口的句柄，而要用编辑区的句柄，否则不会生效！

上例的代码中，如果使用 `hwnd = win32gui.FindWindow('Notepad',None)` 是不会生效的。要改用以下语句：

`hwnd = win32gui.FindWindowEx(win32gui.FindWindow('Notepad',None),None,'Edit',None)`

## RegisterWindowMessage

RegisterWindowMessage 函数定义了一个新的窗口消息，该消息在系统范围内是唯一的。通常调用 SendMessage 或者 PostMessage 函数时可以使用该函数返回的消息值

若成功注册了消息，返回值是一个消息标识符，该标识符范围在 0XC000到0XFFFF 之间。否则，返回值为0。

## 应用程序消息

WM_USER 用于定义专用窗口类使用的专用消息，通常为 WM_USER+x 格式，其中 x 是整数值。

`#define WM_USER        0x0400`

以下是消息编号的范围。

| 范围 | 含义 |
| -- | -- |
| 0 到 WM_USER –1 | 保留供系统使用的消息。|
| 通过 0x7FFF 进行WM_USER | 用于专用窗口类的整数消息。|
| 通过0xBFFF WM_APP (0x8000) | 可供应用程序使用的消息。|
| 通过 0xFFFF 进行0xC000 | 应用程序使用的字符串消息。 |
| 大于 0xFFFF | 系统保留。|

系统定义了第一个范围 (0 到 WM_USER –1) 的消息编号。 未显式定义的此区域中的值由系统保留。

应用程序可以定义 (WM_USER到 0x7FFF) 的第二个范围内的消息编号，并将其用于在专用窗口类中发送消息。 这些值不能用于定义在整个应用程序中有意义的消息，因为某些预定义的窗口类已在此范围内定义值。 例如，预定义的控件类（如 BUTTON、 EDIT、 LISTBOX 和 COMBOBOX ）可以使用这些值。 除非应用程序设计为交换消息并将相同的含义附加到消息号码，否则不应将此范围中的消息发送到其他应用程序。

第三个范围 (0x8000到0xBFFF) 的消息编号可用于应用程序用作专用消息。 此范围中的消息不会与系统消息冲突。

当应用程序调用 RegisterWindowMessage 函数以检索字符串的消息编号时，会在运行时定义第四个范围 (0xC000到0xFFFF) 的消息编号。 注册相同字符串的所有应用程序都可以使用关联的消息号来交换消息。 但是，实际消息编号不是常量，不能假定在不同会话之间相同。

系统保留第五个范围 (大于0xFFFF) 的消息编号。

### 进程间通信 - 注册消息

使用注册消息实现进程间通信。

使用注册消息比较简单，核心是消息的接收端和消息的发送端（接收端和发送端在两个不同的进程）必须注册相同的消息，这样发送消息才能识别。
下面看看具体实现：

## SetWindowSubclass

给当前 HWND 设置子类处理函数. 可以链式执行.
