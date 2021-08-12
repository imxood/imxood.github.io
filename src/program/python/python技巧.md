## 对于开源模块, 不想安装再运行, 可以直接运行

	比如这里的log_server:

		PYTHONPATH="$PYTHONPATH:$(pwd)" python3 -m log_server $@
