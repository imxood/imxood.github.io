# uart笔记

协议分析

https://blog.csdn.net/weixin_44625313/article/details/120015565

## 配置过程

### 中断模式

    选择时钟源

    使能时钟

    配置uart
    
        既读又写: TX_RX
        8字节数据位, 1个开始位, 1个停止位, 无奇偶校验: 8 data bit, 1 start bit, 1 stop bit, no parity
        设置波特率: 115200

    使能uart

    设置中断优先级, NVIC_SetPriority(USARTx_IRQn, 5);  
    使能中断, NVIC_EnableIRQ(USARTx_IRQn);
