# egui 学习

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
