## 给 stage 绑定事件

``` js
PIXI.extensions.remove(PIXI.InteractionManager);

app = new PIXI.Application({
    antialias: true,
    backgroundColor: 0xffffff,
    view: canvas,
});

const { renderer, stage } = app;

renderer.addSystem(EventSystem, "events");

// Make the whole scene interactive
stage.interactive = true;
// Make sure stage captures all events when interactive
stage.hitArea = renderer.screen;

// 事件
stage.addEventListener(
    "wheel",
    (wheel_event: Event) => {
        // e.preventDefault();
    },
    { passive: false }
);
```