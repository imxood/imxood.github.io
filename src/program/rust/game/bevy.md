# bevy 一个游戏引擎

## code

    git clone https://github.com/bevyengine/bevy
    cargo run --example breakout

## state

主要的数据结构:
```rust
// 用于操作游戏state, 比如 设置/替换/退出/进入 state
enum ScheduledOperation<T: Component + Clone + Eq> {
    Set(T),
    Replace(T),
    Pop,
    Push(T),
}

// 状态切换 如果是 PreStartup 则会进入 Startup
// 后面几个是 (leaving, entering), 表示从 leaving状态 切换到 entering状态
enum StateTransition<T: Component + Clone + Eq> {
    PreStartup,
    Startup,
    // The parameter order is always (leaving, entering)
    ExitingToResume(T, T),
    ExitingFull(T, T),
    Entering(T, T),
    Resuming(T, T),
    Pausing(T, T),
}
```

初始化状态, 并添加特定状态下要执行的系统集:
```rust
// 添加一个 state, 即此处的GameState, 设置初始状态为 Playing
// 并添加一个systemset, 设置 run_criteria 为 state_cleaner
App::build()
    .add_state(GameState::Playing)
    .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup.system()))

// 这个函数主要用于处理 state.scheduled 和 state.transition
fn state_cleaner<T: Component + Clone + Eq>(
    mut state: ResMut<State<T>>,
    mut prep_exit: Local<bool>,
)
{
    match state.scheduled.take() {

        Some(ScheduledOperation::Set(next)) => {
            state.transition = Some(StateTransition::ExitingFull(
                state.stack.last().unwrap().clone(),
                next,
            ));
        }
        Some(ScheduledOperation::Replace(next)) => { ... }
        Some(ScheduledOperation::Push(next)) => { ... }
        Some(ScheduledOperation::Pop) => { ... }
        None => match state.transition.take() {
            Some(StateTransition::ExitingFull(p, n)) => { ... }
            Some(StateTransition::Pausing(p, n)) => { ... }
            Some(StateTransition::ExitingToResume(p, n)) => { ... }
            Some(StateTransition::PreStartup) => { ... }
        }
}

// 当执行 state.set(GameState::GameOver), 会设置 scheduled 的值, 就会改变 state.transition 的值, 后面特定的system_set就启动
pub fn set(&mut self, state: T) -> Result<(), StateError> {
    if self.stack.last().unwrap() == &state {
        return Err(StateError::AlreadyInState);
    }

    if self.scheduled.is_some() {
        return Err(StateError::StateAlreadyQueued);
    }

    self.scheduled = Some(ScheduledOperation::Set(state));
    Ok(())
}

```

这会添加两个systemset, 其中的system会按照添加的顺序添加到, 在stage中运行时, 这些systems会自上而下运行

`` .add_state(GameState::Playing) `` 这会初始化状态并放入全局组件中, 会添加一个 systemset, 并带有 state_cleaner 的 run_criteria, 用于处理 判断状态是否满足预期, 是否应该运行.

当执行 ``SystemSet::on_enter``, ``SystemSet::on_exit``, ``SystemSet::on_update``时, 会添加不同的run_criteria, 用于判断特定的状态发生时是否应该执行. 比如:
```rust
pub fn on_update(s: T) -> RunCriteriaDescriptor {
    (|state: Res<State<T>>, pred: Local<Option<T>>| {
        // 当判断当前的state是预期的状态时, 同时事务还未开始时, return true, 表示执行
        state.stack.last().unwrap() == pred.as_ref().unwrap() && state.transition.is_none()
    })
    .system()
    .config(|(_, pred)| *pred = Some(Some(s.clone()))) // 这会把传入的状态即期望的状态保存到这个 system 的第二个参数, 即pred
    .chain(should_run_adapter::<T>.system())
    .after(DriverLabel::of::<T>())
    .label_discard_if_duplicate(StateCallback::Update.into_label(s))
}
```


```rust
// 初始化状态
Self {
    stack: vec![initial],
    transition: Some(StateTransition::PreStartup),
    scheduled: None,
    end_next_loop: false,
}

```

当 systemset 第一次运行时, stage.scheduled 为 None, 则设置 state.transition = Some(StateTransition::Startup), 表示执行Startup事务. 并 设置 prep_exit 为 true, 表示第一次运行完成.

先有 scheduled 再根据 scheduled 判断 transaction



当执行 `` state.set(GameState::GameOver) `` 时, 会验证: 1.当前的 stage.stack 的最后一项不能是要设置的状态 2.stage.scheduled 需要是空的, 然后更新状态: ``self.scheduled = Some(ScheduledOperation::Set(state))``
