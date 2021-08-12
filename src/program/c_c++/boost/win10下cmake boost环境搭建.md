# windows 10 下 cmake boost mingw-w64环境搭建

## 下载mingw-w64

	方法1:

		下载: https://sourceforge.net/projects/mingw-w64/files/Toolchains%20targetting%20Win64/Personal%20Builds/mingw-builds/8.1.0/

		选择 MinGW-W64 GCC-8.1.0 --> x86_64-posix-seh

		解压出去, 把bin目录放到环境变量下, mingw的c++开发环境就算是好了


	方法2:

		choco.exe info mingw --verbose
		choco.exe install mingw

	这两种方法是一样的, choco这个软件管理工具使用的是sourceforge.net/projects/mingw-w64, 这里方法一我只是把choco的下载链接取出来了, 直接使用

## 下载 cmake ninja

	我有python3的环境:

		pip install --user cmake ninja

	没有pip环境, 只好到cmake和ninja的官方网站下载了

## 用 gcc 编译 boost

	我当前用的是cmake 3.18.2, 这个版本会对boost的版本选择造成影响, 3.18.2应该只支持到boost 1.73, 这个是set(Boost_DEBUG ON) cmake的输出可以看出来, 不同版本支持的情况不同, 因为每一个cmake版本都适配了对应的可以find_package的boost版本

	下载 boost_1.73 https://dl.bintray.com/boostorg/release/1.73.0/source

	解压 boost_1.73.0 源码到 D:\temp\boost_1_73_0 目录

	用cmd

	cd D:\temp\boost_1_73_0\tools\build
	.\bootstrap.bat gcc

	mkdir D:\temp\boost-build
	.\b2.exe --prefix="D:\temp\boost-build" install
	set PATH=%PATH%;D:\temp\boost-build\bin;

	cd D:\temp\boost_1_73_0
	b2 --build-dir="D:\temp\boost_1_73_0\build" --build-type=complete --prefix="D:\programs\boost_1_73_0" toolset=gcc install

	最后就会安装到 D:\programs\boost_1_73_0 目录下

	编译环境: I7-8665U windows 10 U盘 Wintogo, 4核8线程, 全速耗时 60 分钟

## cmake example

	cmake_minimum_required(VERSION 3.10)

	project(hello_world)

	# set(CMAKE_CXX_STANDARD 11)

	# 这个Boost_DEBUG是非常有用的, 如果遇到missing的时候请分析输出中"search"相关, 是否是实际的情况
	#     比如: 根据Boost_DEBUG输出, 我判断出没有找到正确的lib, 我编译出的带有arch, 找出的却不带, 另外安装的可能是vc编译的, 我却需要mingw编译的
	set(Boost_DEBUG ON)
	set(Boost_ARCHITECTURE "x64")


	# 设置BOOST的安装路径, 如果设置了BOOST_ROOT环境变量, 这里就不需要了
	# set(BOOST_ROOT "")

	find_package(Boost REQUIRED COMPONENTS filesystem)

	add_executable(hello_world hello_world.cpp)
	target_link_libraries(hello_world Boost::filesystem)
