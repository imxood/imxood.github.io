
## rust语言的工具类

strum

    可以实现 ToString, 字符串 到 枚举 的转换, 枚举遍历, 枚举类型的成员数量, 枚举成员静态消息, 枚举成员静态属性 等等

derive_more

    自动实现 builtin traits, 如 From Into FromStr TryInto IntoIterator AsRef AsMut 等等

rand 随机数

bstr

    实现 \r\n 二进制字符串分割

    let lines: Vec<&[u8]> = data.lines().collect();

sorted_vec

    数组或不重复的集合, 排序

flume

    超快的 mpmc channel实现

## 系统功能

machineid-rs 获取 加密的 独一无二的 MachineID/HWID/UUID

sys-info 专注于获取系统信息, 包括: 主机名, 启动时间, CPU信息, 系统类型, 磁盘使用, 内存使用, 进程数, linux系统 release信息

sysinfo 网络信息(包括网卡, 收发数据量), 内存使用情况, 进程管理, CPU使用情况, 系统信息(系统名,用户名,内核版本,主机名), 等等

portpicker 选择可用网络端口

## 参数解析

clap

structopt
