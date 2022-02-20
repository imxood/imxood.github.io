# winit

EventLoop::new() 创建一个 事件Loop
    它会先创建 platform_impl::EventLoop::new(), 依赖于 每一个特定平台的 事件循环

    platform_impl::EventLoop::new()这个函数中又会 创建EventLoopWindowTarge, 它包含 WindowTarget, 这是个特定平台的 窗口目标

    根据编译环境, 选择不同的 WindowTarget, 这取决于 当前的 编译环境, 如果是 wasm32-unknown-unknown , 则是 web, 如果是 linux, 则是 x11 或 wayland

    每一个 WindowTarget 中是由特定平台实现的

        src/platform_impl/web 这表示一个特定的 platform, 这里表示了 window, monitor 和 event_loop 的逻辑, 就是 管理窗口, 管理显示器, 管理事件.

        src/platform_impl/web/web_sys 这表示 这个platform下的特定 backend, 表示 实现 platform 的底层

        在 web 端的 window, 实际上只是 canvas, 事件管理就是 管理canvas 的所有事件, 所以 浏览器的 window 的 resize事件, 是无法通过winit捕获的