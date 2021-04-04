# HarmonyOS笔记

## stm32f4仿真

### 安装npm
curl -sL https://deb.nodesource.com/setup_14.x | sudo -E bash -
sudo apt-get install -y nodejs

### 安装xpm

	sudo npm install --global xpm
	xpm install --global @xpack-dev-tools/qemu-arm@latest
	xpm install --global @xpack-dev-tools/openocd@latest

### 报错

	sudo dpkg-reconfigure dash
		--> NO

	fatal error: 'alsa/asoundlib.h' file not found
	解决: sudo apt-get install libasound2-dev

	./libavutil/timer.h:37:11: fatal error: 'asm/unistd.h' file not found
	解决: sudo apt-get install libasound2-dev

	sudo apt install pkg-config
