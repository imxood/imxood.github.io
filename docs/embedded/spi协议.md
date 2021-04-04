SPI分析

设置spi参数: polar, phase
	polar, 极性: CPOL=0则, 当SCLK=0时, SPI处于空闲状态; CPOL=1则, 当SCLK=1时, SPI处于空闲状态; 
	phase, 相位: CPHA=0则, 数据采样在第1个边沿, 即上升沿; CPHA=1则, 数据采样在第2个边沿, 即下升沿;

spi一次最多传输FIFO大小的数据, 比如: 64 bytes

要写的缓冲区有: REG BUF (reg_len), WRITE BUF (write_len), READ BUF (read_len)
把上述BUF数据依次放入SPI FIFO:
	reg len --> write len --> read len (写0, dummy)

设置FIFO阈值: FIFO_SIZE / frame_size - 1

使能spi, 开始传输

等待中断...

从FIFO中依次读取所有数据: reg_len(丢弃) --> write_len(丢弃) --> read_len
