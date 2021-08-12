
## ubuntu使用pyenv管理多个python版本

	git clone https://github.com/pyenv/pyenv.git ~/.pyenv

	添加环境:

		export PATH="~/.pyenv/bin:$PATH"
		eval "$(pyenv init -)"

	由于pyenv是下载python源码 编译, 所以安装会用到的库:
		sudo apt-get install -y build-essential libssl-dev zlib1g-dev \
		libbz2-dev libreadline-dev libsqlite3-dev wget curl llvm libncurses5-dev \
		libncursesw5-dev xz-utils tk-dev libffi-dev liblzma-dev python-openssl git

		参考: https://github.com/pyenv/pyenv/wiki/Common-build-problems

	pyenv常用的命令:

		查看支持的python版本:
			pyenv install -l

		安装python3.5版本:
			pyenv install 3.5.10

		查看已管理的所有版本:
			pyenv versions

		对特定应用指定python版本, 会在当前目录下生成'.python-version':
			pyenv local 3.5.10

		全局使用:
			pyenv global 3.5.10

	tips:
		pyenv对tab键非常友好 (for ubuntu)
