## 单片机的hex文件格式

Intel HEX 文件是由一行行符合Intel HEX 文件格式的文本所 构 成的ASCII 文本文件

https://www.eet-china.com/mp/a29853.html


### 04 0000 05 18005B483C

04 4个字节的数据
05 表示开始线性地址, 入口地址

3C 表示前面数据的校验数据, 校验方法: 0x100-前面字节累加和

### 02 0000 04 1800 E2

02 2个字节的数据
0000 起始地址
04 表示扩展线性地址类型, 0x1800 为扩展线性地址

### 10 0000 00 3C3CA5A5C3C35A5A02000000BFFFFFFF36

10 16字节的数据
0000 起始地址
00 数据类型
3C3CA5A5C3C35A5A02000000BFFFFFFF 数据
36 校验码

地址为: 0x18000000