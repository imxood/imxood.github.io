# uart笔记

## 实现 printf

### __GNUC__

```c
int _write(int fd, char *ptr, int len) {
    uart_write(ptr, len);
    return len;
}
```


## 协议分析

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

## UART 发送数据

连续发送多个数据时, 数据寄存器可能会满, 导致剩余数据丢失

应该这么做:

while (1) {
    等待 TX 空闲
    连续发送最大 UART_TX_FIFO_SIZE 数量的数据
}

UART_TX_FIFO_SIZE 可能是 16

## UART 接收数据

UART_RX_FIFO_SIZE 可能是 32

## 使用 RTOS 时

一般是 在中断中 把数据放入到 BUF 中, 在 RX超时 或者 RX到达(人为设置的)阈值时, 发送信号.

在独立的task中 根据信号, copy 出 buf 并处理.

## IDLE中断

总线空闲状态IDLE中断: 当一帧数据传输结束之后，总线会维持高电平空闲，此时会触发MCU的IDLE中断.

可用该中断 处理数据传输完成 事件.
