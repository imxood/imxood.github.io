# AutoGen Studio + Ollama 本地部署

## 说明

- 本页记录 `AutoGen Studio` 与 `Ollama` 的本地联调流程.
- 适合快速搭起一个本地多 Agent / 工作流实验环境.

## 安装 Conda

- 清华镜像说明: <https://mirrors.tuna.tsinghua.edu.cn/help/anaconda/>

常见环境变量路径示例:

```text
C:/Users/maxu/anaconda3
C:/Users/maxu/anaconda3/Scripts
C:/Users/maxu/anaconda3/Library/bin
```

## 创建环境

```sh
conda init
conda create -n autogenstudio python=3.11
conda activate autogenstudio
pip install autogenstudio
autogenstudio ui --port 8081
```

## 接入本地模型

在界面中新增模型时, 可按如下方式配置:

- 模型名: `codestral:22b`
- 接口地址: `http://127.0.0.1:11434/v1`

## 最小联调思路

1. 先启动 `Ollama` 并确认模型可在本地正常响应.
2. 再启动 `AutoGen Studio` 的 Web UI.
3. 在界面里添加模型配置并做一次最小对话验证.
4. 最后再尝试 Agent 模板, 工具调用和多步骤工作流.

## 常见关注点

- 模型名需要与本地 `Ollama` 实际可用模型一致.
- 端口和 API 路径要与本机服务配置一致.
- 若界面能打开但模型不可用, 优先检查本地推理服务是否正常响应.
- 若后续继续整理, 可补 Agent 模板, 工作流设计与常见报错排查.
