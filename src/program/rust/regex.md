

```rs
let re = Regex::new(r"\$\{([_0-9a-zA-Z.]+)\}").unwrap();
for cap in re.captures_iter("aa${var0}bb, aa${var1.a.b}bb") {
    // 输出匹配到的字符串
        eprintln!("{:?}", &cap);
        eprintln!("{:?}", &cap[0]);
        // 输出第一个使用括号捕获的字符串
        eprintln!("{:?}", &cap[1]);

        if let Some(cap) = cap.get(1) {
            eprintln!("cap: {:?} start: {} end: {} str: {}\n", cap, cap.start(), cap.end(), cap.as_str());
        }
}

// 输出:

// Captures({0: 2..9/"${var0}", 1: 4..8/"var0"})
// "${var0}"
// "var0"
// cap: Match { start: 4, end: 8, string: "var0" } start: 4 end: 8 str: var0

// Captures({0: 15..26/"${var1.a.b}", 1: 17..25/"var1.a.b"})
// "${var1.a.b}"
// "var1.a.b"
// cap: Match { start: 17, end: 25, string: "var1.a.b" } start: 17 end: 25 str: var1.a.b

```