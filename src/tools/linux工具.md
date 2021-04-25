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
