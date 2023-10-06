## PCM数据 介绍

https://blog.csdn.net/ZHONGCAI0901/article/details/116131776

## i2s 介绍

https://blog.csdn.net/ZHONGCAI0901/article/details/116205427

(1) bclk, 位时钟, 表示 采样频率 * BitWidth * 声道数量, 对应 有效的数据传输位

bclk 就是 i2s 的 SCLK

(2) ws, 字段(声道)选择, 用于选择左右声道, 0 表示 左声道, 1 表示 右声道

WS 的频率＝采样频率, 就是说, 采样率是 16000Hz, 那么 WS的频率也是 16000

(3) sd：串行数据, 用二进制补码来表示音频数据 (数据传输从高位到低位)
