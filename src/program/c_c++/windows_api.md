# Windows API

## 说明

- 本页整理 `Windows API` 的学习入口和几个高频概念.
- 当前重点是窗口消息循环以及 `SendMessage` / `PostMessage` 的差异.

## 官方学习入口

- Windows 桌面开发教程: <https://learn.microsoft.com/zh-cn/windows/win32/learnwin32/learn-to-program-for-windows>
- Windows 消息循环讲解: <https://learn.microsoft.com/zh-cn/windows/win32/learnwin32/window-messages>

## 消息循环的基本过程

基本过程是:

1. 操作系统在消息队列中放入窗口消息, 例如 `WM_LBUTTONDOWN`.
2. 程序调用 `GetMessage` 获取消息.
3. 消息被填充到 `MSG` 结构中.
4. 程序调用 `TranslateMessage` 和 `DispatchMessage`.
5. 系统进入窗口过程 `WndProc` 处理该消息.
6. 窗口过程可以响应消息或忽略它.

## `SendMessage` / `PostMessage`

参考:

- <https://learn.microsoft.com/zh-cn/windows/win32/api/winuser/nf-winuser-sendmessage>
- <https://blog.csdn.net/mcw_720624/article/details/117192221>

核心区别:

- `SendMessage`: 发送后等待处理结果, 更像同步调用.
- `PostMessage`: 只把消息投递到队列, 不等待处理结果.

适用场景:

- 需要立即获得处理结果时, 更适合 `SendMessage`.
- 只想异步通知目标窗口时, 更适合 `PostMessage`.

## 使用建议

- 做窗口自动化或消息模拟时, 先确认窗口句柄和目标消息类型是否正确.
- 涉及跨线程或跨进程消息时, 要注意同步阻塞和权限问题.
- 若后续继续整理, 可补 `CreateWindow`, `WndProc`, `SetWindowLongPtr` 和窗口枚举相关笔记.
