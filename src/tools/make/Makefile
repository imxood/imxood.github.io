.phony: all

VAR=1

# 每一行是一个 shell 命令, 前面定义了一个 VAR 变量, 同一行的后面可以使用 $$VAR 访问 shell中的变量

all:
	echo $(VAR)
	VAR=2; echo ${VAR}; echo $$VAR
