# linux 工具

## flameshot 截图工具

命令: flameshot gui, 进入截图

## DSView 逻辑分析器

### 编译

	git clone https://github.com/DreamSourceLab/DSView.git

	sudo apt-get install git-core build-essential cmake autoconf automake libtool pkg-config \
	libglib2.0-dev libzip-dev libudev-dev libusb-1.0-0-dev \
	python3-dev qt5-default libboost-dev libboost-test-dev libboost-thread-dev libboost-system-dev libboost-filesystem-dev check libfftw3-dev

	cd libsigrok4DSL
	./autogen.sh
	./configure
	make -j
	sudo make install
	cd ..

	cd libsigrokdecode4DSL
	./autogen.sh
	./configure
	make -j
	sudo make install
	cd ..

	cd DSView
	mkdir build -p && cd build && cmake ..
	make -j
	sudo make install


## wps 中文字体部分无显示

[添加字体， 更新字体缓存](https://mxy493.xyz/2019040840601/) 需要重启生效(或登出?)

## Samba 共享

安装

    sudo apt-get install samba

修改 /etc/samba/smb.cof

    [profiles]
    comment = Share Folder
    path = /develop/share
    guest ok = yes
    browseable = yes
    public = yes
    writable = yes
    force users = nobody
    force group = nogroup
    force create mode = 0775
    force directory mode = 0775
