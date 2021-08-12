# AT&T汇编基础

[非常实用的AT&T教程](https://www.jianshu.com/p/4481c2aeac6b)
[Assembly_In_Linux](http://www.kerneltravel.net/newbie/Assembly_In_Linux.pdf)

寄存器前加 %
立即数前加 $

### mov指令

    mov的操作指令可用于以下类型的传送：

        立即数传送给通用寄存器
        立即数传送给内存
        通用寄存器传送给另一个通用寄存器
        通用寄存器传送给段寄存器
        段寄存器传送给通用寄存器
        通用寄存器传送给控制寄存器
        控制寄存器传送给通用寄存器
        通用寄存器传送给调试寄存器
        调试寄存器传送给通用寄存器
        内存位置传送给通用寄存器
        内存位置传送给段寄存器
        通用寄存器传送给内存位置
        段寄存器传送给内存位置

### 索引寻址(变址寻址)

    语法: disp(base,index,scale)

    即公式:  [base+index*scale+disp]

    如:
        Movl 0x20(%ebx),%eax
        Addl (%ebx,%ecx,0x2),%eax
        Leal (%ebx,%ecx),%eax
        Subl -0x20(%ebx,%ecx,0x4),%eax
