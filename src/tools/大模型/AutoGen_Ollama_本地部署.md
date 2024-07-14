1. 安装 conda

https://mirrors.tuna.tsinghua.edu.cn/help/anaconda/

并配置 conda环境变量

C:\Users\maxu\anaconda3
C:\Users\maxu\anaconda3\Scripts
C:\Users\maxu\anaconda3\Library\bin

2. 使用 conda 配置环境

```sh
# conda 安装好后, 第一次似乎需要先执行一下 conda初始化?
conda init

conda create -n autogenstudio python=3.11

conda activate autogenstudio

# conda deactivate

pip install autogenstudio

autogenstudio ui --port 8081

```

3. 打开连接

新建模型:

codestral:22b
http://127.0.0.1:11434/v1

新建 2个 Agent:
