
## 在 stm32cubemx 中添加软件包

https://www.rt-thread.org/download/cube/RealThread.RT-Thread.pdsc

## 基本配置

设置时钟:

![](docs/nano使用笔记/2022-06-27-10-37-48.png)

![](docs/nano使用笔记/2022-06-27-12-45-09.png)

设置 Debug 模式为 SW方式, 设置 时间基为 Tim4(随意选的一个)

![](docs/nano使用笔记/2022-06-27-10-38-16.png)

添加 rtt nano 包

![](docs/nano使用笔记/2022-06-27-10-41-11.png)

![](docs/nano使用笔记/2022-06-27-10-41-39.png)

生成项目, 这里使用的是 gcc + Makefile

![](docs/nano使用笔记/2022-06-27-10-42-19.png)

## 修改一些文件

Makefile

![](docs/nano使用笔记/2022-07-01-09-11-15.png)

![](docs/nano使用笔记/2022-07-01-09-12-21.png)

![](docs/nano使用笔记/2022-06-27-10-45-04.png)

修改链接脚本, 使 rt 的初始化可以正常被执行
![](docs/nano使用笔记/2022-06-27-12-41-09.png)

去掉几个默认生成的中断函数
![](docs/nano使用笔记/2022-06-27-12-53-55.png)

## ws2812 控制

可以参考: https://www.cnblogs.com/dongxiaodong/p/14362196.html

