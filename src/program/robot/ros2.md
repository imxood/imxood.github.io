## 安装 ros2 参考视频

https://www.youtube.com/watch?v=mL6cDcnLTas&ab_channel=EraBot

https://rulshi.blogspot.com/2024/02/how-to-install-ros2-in-windows-11.html

仅仅简单的安装 ros2 release, 配置好环境变量后

加载环境 local_setup.ps1

验证环境

ros2 run demo_nodes_py talker

如果失败, 如: failed to load any RMW implementations

需要安装 openssl v1.1.1: choco install -y openssl --version 1.1.1.2100

可以不安装那么多的环境
