# dma笔记

cpu对内存的读写, 由于速度问题, 需要高速cache

cpu读操作: 先从cache中取值,若命中, 则返回,

如果接收到的数据为空, 则可能数据有cache, 需要清除cache
