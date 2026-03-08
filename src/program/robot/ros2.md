# ROS2

## 说明

- 本页记录 `ROS2` 的 Windows 安装后验证和常见环境问题.
- 适合作为“先把环境跑通”的最小入口页.

## 参考资料

- 视频: <https://www.youtube.com/watch?v=mL6cDcnLTas&ab_channel=EraBot>
- Windows 安装参考: <https://rulshi.blogspot.com/2024/02/how-to-install-ros2-in-windows-11.html>

## 基本安装后验证

- 配置好环境变量后, 先加载 `local_setup.ps1`.
- 使用以下命令验证环境:

```sh
ros2 run demo_nodes_py talker
```

若终端能正常启动示例节点, 说明基础运行时已基本可用.

## 建议的最小检查项

1. 确认 Python, Visual C++ 运行库和 ROS2 本体均已安装.
2. 确认当前终端已加载 ROS2 的环境脚本.
3. 先跑 `demo_nodes_py` 或 `demo_nodes_cpp` 做最小验证.
4. 再进入工作区, 包创建和节点通信实验.

## 常见问题

### `failed to load any RMW implementations`

可尝试安装 `openssl 1.1.1`:

```sh
choco install -y openssl --version 1.1.1.2100
```

排查方向:

- 检查 `RMW` 实现是否正确安装.
- 检查环境变量是否在当前终端生效.
- 检查依赖库版本是否与当前 ROS2 发行版兼容.

## 后续可补主题

- 工作区创建与 `colcon build`.
- 包管理和依赖声明.
- `topic`, `service`, `action` 的最小示例.
- 常见中间件配置与跨机通信验证.
