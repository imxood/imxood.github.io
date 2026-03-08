# Rust 索引性能讨论

## 原始讨论

- Reddit: https://old.reddit.com/r/rust/comments/1rhb97r/is_there_any_significant_performance_cost_to/
- 主题: `array.get(idx).ok_or(Error::Whoops)` 和 `array[idx]` 的性能差异
- 发帖时间: 2026-02-28 19:15:51 UTC, 即北京时间 2026-03-01 03:15:51

## 问题核心

原帖主要在问 3 件事:

1. `array.get(idx).ok_or(Error::Whoops)` 相比 `array[idx]` 是否有明显性能损耗.
2. `array.get(idx).ok_or(Error::Whoops)` 是否比手写 `if idx < array.len()` 更快.
3. 在大量索引访问, 但又不方便改成迭代器时, 应该优先选择哪种写法.

## 讨论中的高价值结论

### 1. 大多数情况下, `.get(idx)` 和 `array[idx]` 都要做边界检查

讨论里比较一致的观点是: 只要编译器不能证明索引一定合法, 这两种写法通常都需要边界检查.

- `array[idx]` 的越界结果是 panic.
- `array.get(idx)` 的越界结果是 `None`.
- 再接 `ok_or(...)` 或 `ok_or_else(...)` 时, 只是把 `Option` 转成 `Result`.

也就是说, 真正决定开销的核心往往不是语法表面, 而是:

- 编译器能不能消掉边界检查.
- 错误对象是否需要提前构造.
- 这段代码是不是热路径.

### 2. `ok_or_else(...)` 通常比 `ok_or(...)` 更合适

这是讨论里最明确, 也最有实用价值的一点.

Rust 官方文档明确说明:

- `Option::ok_or(err)` 会急切地计算参数 `err`.
- `Option::ok_or_else(f)` 会延迟到 `None` 时才调用闭包构造错误.

因此, 如果错误值构造并不便宜, 或错误类型带有析构成本, 更推荐写成:

```rust
array.get(idx).ok_or_else(|| Error::Whoops)
```

这可以避免在成功路径上白白创建错误对象.

### 3. 手写 `if` 一般不会天然比 `.get(...).ok_or_else(...)` 更快

讨论中有人用 Compiler Explorer 对比汇编, 结论是:

- 在开启优化后, 这几种写法经常会被内联成非常接近的机器码.
- 如果编译器已经能看懂控制流, 手写 `if` 未必有额外优势.

所以, 对大部分普通代码来说:

- 不要凭感觉假设某种写法一定更快.
- 先写可读性更好的版本.
- 真进入性能热点后, 再基于 benchmark 或汇编做决定.

### 4. `array[idx]` 更适合表达 "这里按不变式保证一定合法"

讨论里也有比较稳妥的经验法则:

- 如果索引一定合法, 比如常量索引, 前面已经检查过, 或由严格不变式保证, 可以使用 `array[idx]`.
- 如果索引可能非法, 或你更想把错误显式传递出去, 用 `.get(idx)` 更安全, 语义也更清楚.

这其实是在性能之外, 进一步强调 API 语义:

- `[]` 偏向 "越界就是逻辑错误".
- `.get()` 偏向 "越界是可恢复分支".

### 5. 真想绕过边界检查, 通常只有 `get_unchecked`

讨论里也提到, 如果目标是彻底避免边界检查, 最直接的方式通常是 `get_unchecked`.

但这意味着:

- 进入 `unsafe`.
- 越界会变成未定义行为, 而不是 panic 或 `None`.
- 只有在 profiler 已经证明这里是关键热点, 且你能严格证明索引合法时, 才值得考虑.

换句话说, `get_unchecked` 不是 `.get()` 和 `[]` 的日常替代品, 而是性能极限优化手段.

## 结合 Rust 文档与源码的补充分析

### `ok_or` 与 `ok_or_else`

Rust 官方文档已经明确指出:

- `ok_or` 是 eager 的.
- `ok_or_else` 是 lazy 的.

因此, 原帖里的写法如果要继续保留 `Result` 风格, 更建议替换为:

```rust
let value = array
    .get(idx)
    .ok_or_else(|| Error::Whoops)?;
```

### `array[idx]` 最终也会走到 slice 索引逻辑

从 Rust 标准库源码可以看到:

- 数组的索引实现会委托给 slice.
- slice 的 `get` 和索引逻辑底层都依赖 `SliceIndex`.

这说明讨论里的判断是合理的: 它们在抽象层面不同, 但底层都离不开 "索引合法性检查" 这个核心问题.

## 实用建议

### 日常代码

优先写可读, 安全, 语义清晰的版本:

```rust
let value = array
    .get(idx)
    .ok_or_else(|| Error::Whoops)?;
```

适用场景:

- 越界属于正常错误分支.
- 想把错误向上传递.
- 更在意代码意图清晰, 而不是极限微优化.

### 已知索引一定合法

可以直接写:

```rust
let value = array[idx];
```

适用场景:

- 索引已经在前面检查过.
- 索引由数据结构不变式保证.
- 越界一旦发生, 就是程序逻辑 bug.

### 性能极端敏感路径

按这个顺序处理:

1. 先用 benchmark 或 profiler 确认这里真是热点.
2. 看 release 汇编, 不要凭直觉优化.
3. 只有在收益明确时, 再考虑 `unsafe get_unchecked`.

## 一句话结论

对这个 Reddit 帖子的结论做一句话总结:

`array.get(idx).ok_or(Error::Whoops)` 通常不会比 `array[idx]` 慢到值得担心, 但应优先改成 `ok_or_else(...)`; 真正的差异更多取决于错误构造成本, 编译器能否消除边界检查, 以及这段代码是不是热点.

## 参考链接

- Reddit 原帖: https://old.reddit.com/r/rust/comments/1rhb97r/is_there_any_significant_performance_cost_to/
- Rust `Option` 文档: https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or_else
- Rust 数组索引源码: https://doc.rust-lang.org/src/core/array/mod.rs.html#382-384
- Rust slice 索引源码: https://doc.rust-lang.org/src/core/slice/index.rs.html#214-220
