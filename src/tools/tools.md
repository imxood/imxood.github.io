# 好用的工具

## vscode

    方法1:
        下载 [vscode insiders 体验版](https://code.visualstudio.com/insiders/)

    方法2:
        curl https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor > packages.microsoft.gpg
        sudo install -o root -g root -m 644 packages.microsoft.gpg /usr/share/keyrings/
        sudo sh -c 'echo "deb [arch=amd64 signed-by=/usr/share/keyrings/packages.microsoft.gpg] https://packages.microsoft.com/repos/vscode stable main" > /etc/apt/sources.list.d/vscode.list'

## flameshot, linux下的截图软件

    # Compile-time
    sudo apt install g++ build-essential qt5-default qt5-qmake qttools5-dev-tools

    # Run-time
    sudo apt install libqt5dbus5 libqt5network5 libqt5core5a libqt5widgets5 libqt5gui5 libqt5svg5-dev

    # Optional
    sudo apt install git openssl ca-certificates

    # Build
    mkdir build && cd build
    qmake ../
    make
    sudo make install

## kicad 电路设计软件

## 立创EDA

## universalindentgui

支持各种代码格式化工具的可视化配置,并预览

## ubuntu dconf-editor, 提供gui设置gsettings

## scp, 类型winscp, 图形化scp

## gnome tweaks工具, 桌面定制工具

sudo apt install -y gnome-shell-extensions gnome-tweak-tool

## 配置中文环境

    locale 查看当前环境
    locale -a 查看可使用的环境列表
    sudo apt install language-pack-zh-hans 安装中文语言包
    sudo bash -c 'echo "LC_ALL=zh_CN.UTF-8" >> /etc/default/locale'

    sudo apt install curl

## 类似ps的绘图软件: gimp

    sudo add-apt-repository ppa:otto-kesselgulasch/gimp
    sudo apt install gimp

## 安装gparted: 磁盘管理,开机引导

    sudo apt install gparted
    sudo gparted

## 安装谷歌浏览器，并使用代理

    sudo apt install chromium-browser
    chromium-browser --proxy-server=socks5://127.0.0.1:1080

## 安装virtualbox

	ubuntu:
		sudo apt install virtualbox virtualbox-ext-pack virtualbox-guest-additions-iso
		sudo usermod -a -G vboxusers peak
		sudo reboot now

	virtualBox虚拟机报错NS_ERROR_FAILURE, 可能是刚升级了内核,virtualBox的内核驱动没有成功被加载, 重启virtualbox服务即可:
	sudo systemctl restart virtualbox.service

	下载地址:https://www.virtualbox.org/wiki/Downloads
	看不到usb3设备?下载下面的插件:
	http://download.virtualbox.org/virtualbox
	【管理】->【全局设定】->【扩展】,选择扩展文件后,就安装成功了
	还要在【usb设备】中选择usb3.0控制器

	下载了usb3的驱动(不知道是否真的需要):http://www.displaylink.com/downloads/ubuntu
	下面三个需要测一测是否需要,我是添加了:
	sudo groupadd usbfs
	sudo adduser maxu vboxusers
	sudo adduser maxu usbfs

	重启本机之后,就发现有了

## 流量监控软件

    https://www.linuxidc.com/Linux/2018-01/150604.htm
    sudo apt install nethogs
    sudo apt install iftop
    sudo apt install vnstat

    sudo nethogs
    显示所有应用程序及其进程号及网速

    用命令iftop来检查带宽使用情况. netstat用来查看接口统计报告,还有top监控系统当前运行进程.
    但是如果你想要找一个能够按进程实时统计网络带宽利用率,那么NetHogs就是你所需要的唯一工具

### 安装openvpn

    sudo apt install openvpn -y

### centos安装lsb_release

    yum provides */lsb_release
    yum install -y lsb_release

### centos安装nginx

    vim /etc/yum.repos.d/nginx.repo
    yum clean all && yum makecache
    yum install -y nginx

### aria2使用

    sudo apt -y install aria2
    sudo yum install aria2 -y

    aria2c --conf-path=<PATH>

    JSON-RPC Path:
        http://token:123@localhost:6800/jsonrpc

### apt设置代理

    sudo vim /etc/apt/apt.conf:
        Acquire::http::Proxy "http://127.0.0.1:8123";
        Acquire::https::Proxy "http://127.0.0.1:8123";

## 编译amule

    yum install -y make automake autoconf gettext zlib-devel wxGTK-devel gcc gcc-c++ kernel-headers binutils-devel bison libupnp

	先编译crypto++:
	git clone https://github.com/weidai11/cryptopp.git
	cd cryptopp
	make -j4
	sudo make install

	可以选择编译libpng:
	git clone https://github.com/glennrp/libpng.git
	cd libpng
	./autogen.sh
	mkdir out && cd out
	../configure
	make -j4
	sudo make install

	wget https://jaist.dl.sourceforge.net/project/amule/aMule/2.3.2/aMule-2.3.2.tar.xz
	tar -xvf aMule-2.3.2.tar.xz
	cd aMule-2.3.2
	./autogen.sh
	mkdir out && cd out
	../configure --enable-profile --enable-optimize --enable-geoip --enable-wxcas --enable-mmap --with-boost --enable-wxcas --enable-cas --enable-alcc --enable-alc --enable-amule-daemon --enable-amulecmd --enable-amule-gui --enable-webserver
	make -j4

	报错:
	ClientCreditsList.cpp:315:10: error:
	.... has no member named ‘DEREncode’
	pubkey.DEREncode(asink);
	解决:
	https://github.com/amule-project/amule/pull/120/commits/d1d1368c7909ffd8423730afaa811ce7b6a3a8aa

	make -j4

## ubuntu安装dia做流程图

	sudo apt install dia-common

## Dia 无法输入中文

	用 vim 打开 /usr/bin/dia 将内容改成下面这样
	dia-normal "$@"

## ssh工具 putty

    sudo apt install putty -y

## gcp 拷贝工具

## audacity 声音录制

## 串口工具

    cutecom picocom

## chrome 插件: 百度下载工具

    baidu-dl
    执行: aria2c --enable-rpc

## ubuntu nvidia graphics drivers

    sudo apt purge nvidia*
    sudo add-apt-repository -y ppa:graphics-drivers && sudo apt update
    sudo apt install nvidia-410

## ubuntu bison flex

    sudo apt install bison flex

## 源码安装qemu

    git clone https://git.qemu.org/git/qemu.git
    cd qemu
    git submodule init
    git submodule update --recursive
    ./configure
    make -j

    # 参考https://wiki.qemu.org/Hosts/Linux
    sudo apt purge "qemu*" && sudo apt autoremove -y && sudo apt install checkinstall -y && sudo apt build-dep qemu -y
    下载[qemu-4.0.0](https://download.qemu.org/qemu-4.0.0.tar.xz)
    tar -xvf qemu-4.0.0.tar.xz && cd qemu-4.0.0
    ./configure --enable-debug && make -j11
    sudo checkinstall make install
    # sudo apt install ./*.deb


## 视频录制

sudo add-apt-repository ppa:obsproject/obs-studio
sudo apt update
sudo apt install obs-studio


## 远程桌面

	https://wiki.x2go.org/doku.php/doc:installation:start

	服务端:
		sudo add-apt-repository ppa:x2go/stable
		sudo apt-get update
		sudo apt install x2goserver x2goserver-xsession

		安装后会自动启动服务:
			systemctl status x2goserver.service

		如果 backend 报错:

			设置 backend 为sqlite:
				vim /etc/x2go/x2gosql/sql, 添加一行:
					backend=sqlite

			或者使用 PostgreSQL 作为 backend:

				sudo -u postgres psql

				postgres=# alter user postgres with password '123456';

				sudo vim /etc/x2go/x2gosql/passwords/pgadmin, 添加密码:
					123456

				sudo x2godbadmin --adduser maxu

				sudo x2godbadmin --listuser
				sudo x2godbadmin --createdb

			重启服务:
				sudo systemctl start x2goserver.service

			实时查看:
				sudo journalctl -u x2goserver.service -f

	客户端:
		sudo apt install x2goclient

		https://wiki.x2go.org/doku.php/doc:installation:x2goclient


		如果无法连接, 可以启动debug模式

			服务器端先关掉服务, 然后用debug模式启动:
				/usr/sbin/x2gocleansessions --debug

		客户端启动debug模式:
			./x2goclient.debug.exe --debug 或 x2goclient --debug

		遇到过错误:
			ubuntu20 安装 x2goserver-xsession 时, 少了 /etc/x2go/Xsession, 从cache中的deb文件中解压出一份就可以

## Typora markdown

官网 https://typora.io

	# or run:
	# sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys BA300B7755AFCFAE
	wget -qO - https://typora.io/linux/public-key.asc | sudo apt-key add -
	# add Typora's repository
	sudo add-apt-repository 'deb https://typora.io/linux ./'
	sudo apt-get update
	# install typora
	sudo apt-get install typora


## Pandoc

markdown的瑞士军刀, 支持输出 PDF、EPUB、HTML 幻灯片等多种格式

	pandoc demo.md -o demo.html

	pandoc <files> <options>

	pandoc --list-input-formats
		commonmark
		creole
		docbook
		docx
		epub
		fb2
		gfm
		haddock
		html
		jats
		json
		latex
		man
		markdown
		markdown_github
		markdown_mmd
		markdown_phpextra
		markdown_strict
		mediawiki
		muse
		native
		odt
		opml
		org
		rst
		t2t
		textile
		tikiwiki
		twiki
		vimwiki

	pandoc --list-output-formats
		asciidoc
		beamer
		commonmark
		context
		docbook
		docbook4
		docbook5
		docx
		dokuwiki
		dzslides
		epub
		epub2
		epub3
		fb2
		gfm
		haddock
		html
		html4
		html5
		icml
		jats
		json
		latex
		man
		markdown
		markdown_github
		markdown_mmd
		markdown_phpextra
		markdown_strict
		mediawiki
		ms
		muse
		native
		odt
		opendocument
		opml
		org
		plain
		pptx
		revealjs
		rst
		rtf
		s5
		slideous
		slidy
		tei
		texinfo
		textile
		zimwiki


## Cura 3D打印软件

	https://github.com/Ultimaker/Cura


## Openpnp 自动贴片

<<<<<<< HEAD
	https://openpnp.org

## Sourcetrail 源码阅读工具

	https://github.com/CoatiSoftware/Sourcetrail

## 词典 goldendict

	sudo apt install qt5-default

	sudo apt-get install git pkg-config build-essential qt5-qmake \
     libvorbis-dev zlib1g-dev libhunspell-dev x11proto-record-dev \
     qtdeclarative5-dev libxtst-dev liblzo2-dev libbz2-dev \
     libao-dev libavutil-dev libavformat-dev libtiff5-dev libeb16-dev \
     libqt5webkit5-dev libqt5svg5-dev libqt5x11extras5-dev qttools5-dev \
     qttools5-dev-tools qtmultimedia5-dev libqt5multimedia5-plugins

	git clone https://github.com/goldendict/goldendict

	cd goldendict
	qmake
	make -j40

	添加欧路在线词典：
		https://dict.eudic.net/dicts/en/%GDWORD%

	# Google Translate -> [Chinese]
	trans -e google -s auto -t zh-CN -show-original y -show-original-phonetics n -show-translation y -no-ansi -show-translation-phonetics n -show-prompt-message n -show-languages y -show-original-dictionary n -show-dictionary n -show-alternatives n "%GDWORD%"

	# Google Translate -> [English]
	trans -e google -s auto -t en-US -show-original y -show-original-phonetics n -show-translation y -no-ansi -show-translation-phonetics n -show-prompt-message n -show-languages y -show-original-dictionary n -show-dictionary n -show-alternatives n "%GDWORD%"
=======
https://openpnp.org

## python 库

	jsoncomment, 可以解析带有逗号结尾的json内容
>>>>>>> 8ec77846e4f205a9e6edaefc35b9c063f708ce64
