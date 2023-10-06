可以对系统的 Code Flash 分出一个 可读写 的区域, 但需要注意 4K 扇区对齐(否则, 应该是起不来程序吧?)

根据 ch32v307 的数据手册, 通过存储器地址映射, 确定可以操作的地址

```
FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 256K
/* 0x40800 ~ 48800 */
USER (rw) : ORIGIN = 0x0040800, LENGTH = 32K
```

USER区域 FLASH操作地址是: 0x8040800 ~ 0x8048800

FLASH 操作需要 unlock, 擦除, 写入, lock
