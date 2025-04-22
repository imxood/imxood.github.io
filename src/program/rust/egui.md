# egui 学习

### 2022 4.13

根据不同的布局, 比如: 自上到下布局中，组件 自左到右/居中/自右到左 放置， 这里涉及到一个 frame_rect 和 widget_rect，分别表示 组件需要的空间大小, 组件本身的大小(包含margin)

还有一个justify, 这个参数用来控制一个组件是否需要铺满这个宽或高, 就是 widget_rect 的宽或高是满的

如果是垂直布局 水平居中对齐 或 水平布局 垂直居中对齐, 则 frame_rect 的宽或高是满的

拉伸边界线 无法让panel变宽, 原因是 拉伸改变了 ui.placer.region 的 max_rect, 但是最终生成的 Shape 是根据当前ui 的 ui.placer.region.min_rect，（根据代码中min_rect的注释, 有点难理解它的意思）
每一次分配一个新空间 (最终调用 allocate_space_impl), 会根据布局计算出需要的Rect, 然后修改 ui.placer.region的 min_rect 和 max_rect， 再移动 cursor 以确定下一次分配空间的位置.

### 2022 4.12 笔记

当实际分配的宽度不够某个窗口时, 窗口就会收缩, 而不管窗口有多大的宽度

关于 frame_rect 和 widget_rect,

frame_rect 表示: 根据布局得到一个空间, 用于放置 widget.
widget_rect 表示: 一个 widget 的实际形状.

    比如说: 水平居中布局, 这个 frame_rect 的宽度 等于 ui 的有效宽度


### 2022 3.13 笔记

当执行 App 的 update时, CtxRef对象 已经从Input得到了足够的数据, 比如获取到了 屏幕尺寸 鼠标移动 按键 等等.

当执行 ``` egui::SidePanel::right("side_panel").min_width(250.0) ``` 时, 只是构建了一个基本的 Panel对象,
当执行 ``` panel.show(ctx, |ui|{}) ``` 时, 会根据 当前有效区域 和 屏幕尺寸 构建一个 Ui 对象. panel 的layer_id 是 LayerId::background()
Ui对象中, 会构建一个 Painter对象, Placer对象

``` Placer::new(max_rect, Layout::default()) ``` 根据当前有效区域， 默认布局是 从上到下, 向左对齐,

Layout

    方向:
        从上到下
        从下到上
        从左到右
        从右到左

    对齐:
        Min: 左或上
        Center: 水平或垂直居中
        Max: 右或下

    main_dir 主轴方向

    main_wrap, 当到达主轴方向结束的地方时, 就截断

    main_align, 主轴上 对齐方向

### 铺满规则

    宽度铺满, 两种情况:
        1. 主轴垂直 且 水平方向 居中对齐
        2. 主轴水平 且 主轴的justify 为true

    高度铺满, 两种情况:
        1. 主轴水平 且 垂直方向 居中对齐
        2. 主轴垂直 且 主轴的justify 为true

### Ui对象

    每次创建Ui对象时, ui的有效区域已经确定好了,

        ui.max_rect() 表示 ui的有效区域

    layout, 包含了 组件的排列方式, 比如: 从上到下? 超出截断? 高度或宽度铺满?

    placer, 包含了 min_

## 选中时 显示边框

参考 crates\egui\src\widgets\selected_label.rs

##

ScrollArea::vertical(), 宽度是自适应的, 如果需要父容器