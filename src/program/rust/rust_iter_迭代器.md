## 小技巧

length - index 表示: 从 index 开始到结尾的长度,,, 一般可用来表示剩余长度

index - length 表示:

index + length 表示 从 index 开始 长度为 length 的下一个位置,,, 开区间, 可以表示结束位置, 但不包含这个位置

## zip 把两个 vec 打包

## chain 链式两个可迭代的对象

## flatten 把多个相同的数组 展开到一个数组中

let data = vec![vec![1, 2, 3, 4], vec![5, 6]];
let flattened = data.into_iter().flatten().collect::<Vec<u8>>();
assert_eq!(flattened, &[1, 2, 3, 4, 5, 6]);

## flod 求和

data.iter().fold(0u32, |acc, &x| acc + x as u32)

##

let mut v = Vec::new();
for (a, b) in (1..5).tuple_windows() {
v.push((a, b));
}
assert_eq!(v, vec![(1, 2), (2, 3), (3, 4)]);

##

let output = raw.chars().collect::<Vec<char>>();
let output = output.chunks(count).map(|v| v.iter().collect::<String>());

result = if invert {
output.rev().collect::<Vec<String>>().join(&add_str)
} else {
output.collect::<Vec<String>>().join(&add_str)
};

##

let result = "abcdef";
result.chars().collect::<Vec<char>>().windows(3).for_each(|v| {
println!("{v:?}");
});

输出:
['a', 'b', 'c']
['b', 'c', 'd']
['c', 'd', 'e']
['d', 'e', 'f']

##

let result = "abcdefg";
result.chars().collect::<Vec<char>>().chunks(3).for_each(|v| {
println!("{v:?}");
});

输出:

['a', 'b', 'c']
['d', 'e', 'f']
['g']
