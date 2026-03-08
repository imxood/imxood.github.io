# Rust 迭代器技巧

## 说明

- 本页整理 Rust 迭代器与集合处理中常见的小技巧和代码片段.
- 适合作为 `zip`, `chain`, `flatten`, `fold`, `windows`, `chunks` 等方法的速查页.

## 小技巧

- `length - index` 常表示“从 `index` 到结尾还剩多少长度”.
- `index - length` 常表示“从当前位置回退指定长度后的起点”, 使用时要注意不要越界.
- `index + length` 常表示“从 `index` 开始, 长度为 `length` 的结束位置”, 通常是一个不含终点的开区间边界.

## zip 把两个 vec 打包

- 适合把两个等长或可并行消费的序列组合成元组流.

## chain 链接两个可迭代对象

- 适合把两个同类型迭代器首尾串起来统一处理.

## flatten 展开嵌套集合

```rust
let data = vec![vec![1, 2, 3, 4], vec![5, 6]];
let flattened = data.into_iter().flatten().collect::<Vec<u8>>();
assert_eq!(flattened, &[1, 2, 3, 4, 5, 6]);
```

## fold 求和

```rust
data.iter().fold(0u32, |acc, &x| acc + x as u32)
```

## tuple_windows 生成相邻窗口

```rust
let mut v = Vec::new();
for (a, b) in (1..5).tuple_windows() {
    v.push((a, b));
}
assert_eq!(v, vec![(1, 2), (2, 3), (3, 4)]);
```

## chunks 后分组拼接字符串

```rust
let output = raw.chars().collect::<Vec<char>>();
let output = output.chunks(count).map(|v| v.iter().collect::<String>());

result = if invert {
    output.rev().collect::<Vec<String>>().join(&add_str)
} else {
    output.collect::<Vec<String>>().join(&add_str)
};
```

## windows 滑动窗口

```rust
let result = "abcdef";
result.chars().collect::<Vec<char>>().windows(3).for_each(|v| {
    println!("{v:?}");
});
```

输出:

```text
['a', 'b', 'c']
['b', 'c', 'd']
['c', 'd', 'e']
['d', 'e', 'f']
```

## chunks 固定分块

```rust
let result = "abcdefg";
result.chars().collect::<Vec<char>>().chunks(3).for_each(|v| {
    println!("{v:?}");
});
```

输出:

```text
['a', 'b', 'c']
['d', 'e', 'f']
['g']
```
