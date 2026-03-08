# Regex

## 说明

- 本页记录 Rust `regex` 库中常用的匹配, 捕获与替换模式.
- 适合处理模板变量, 占位符和结构化字符串提取.
- 如果你的目标是轻量文本扫描, `regex` 往往比手写字符串切分更稳定.

## 依赖

```toml
[dependencies]
regex = "1"
```

## 基础示例: 提取 `${...}` 占位符

```rust
use regex::Regex;

let re = Regex::new(r"\$\{([_0-9a-zA-Z.]+)\}").unwrap();

for cap in re.captures_iter("aa${var0}bb, aa${var1.a.b}bb") {
    println!("full = {}", &cap[0]);
    println!("name = {}", &cap[1]);

    if let Some(group) = cap.get(1) {
        println!(
            "start = {}, end = {}, value = {}",
            group.start(),
            group.end(),
            group.as_str()
        );
    }
}
```

## 这个例子做了什么

- 匹配 `${...}` 形式的占位符.
- 第 `0` 组是完整匹配结果, 例如 `${var0}`.
- 第 `1` 组是括号内捕获内容, 例如 `var0` 或 `var1.a.b`.
- 通过 `cap.get(1)` 可以拿到位置范围和原始字符串切片.

## 常见 API

### `is_match`

```rust
let re = Regex::new(r"^\d+$").unwrap();
assert!(re.is_match("12345"));
```

- 只关心“是否匹配”时最直接.

### `find_iter`

```rust
let re = Regex::new(r"\d+").unwrap();
for item in re.find_iter("a1 b22 c333") {
    println!("{}", item.as_str());
}
```

- 适合只提取命中片段, 不需要捕获组时使用.

### `captures_iter`

- 适合需要按组提取结构化内容时使用.
- 模板变量, 路径参数和局部语法解析都很常见.

### `replace_all`

```rust
use regex::Regex;

let re = Regex::new(r"\$\{([_0-9a-zA-Z.]+)\}").unwrap();
let output = re.replace_all("hello ${name}", |caps: &regex::Captures| {
    match &caps[1] {
        "name" => "Rust".to_string(),
        other => format!("<missing:{other}>"),
    }
});

assert_eq!(output, "hello Rust");
```

- 适合做模板替换和批量文本重写.

## 使用建议

- 正则表达式应尽量预编译并复用, 不要在高频循环里反复 `Regex::new`.
- 模式复杂时, 优先先写测试样例, 再逐步补充分组.
- 如果只是判断前后缀或简单切分, 可以优先用字符串 API, 不必上正则.

## 常见注意点

- Rust `regex` 不支持回溯引用和环视这类 PCRE 风格高级特性.
- 想提高可读性时, 可以把复杂模式拆分到多个小正则或先做预过滤.
- 对用户输入做替换时, 要明确区分“匹配模式”和“替换模板”两侧的转义语义.

## 适用场景

- 模板变量替换.
- 配置文件中的占位符扫描.
- 日志, 路径或协议字段中的局部提取.
