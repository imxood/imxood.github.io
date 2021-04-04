## 搭建环境

    TensorFlow Lite: https://www.tensorflow.org/lite/microcontrollers

    搭建参考: https://github.com/tensorflow/tensorflow/tree/master/tensorflow/lite/micro/examples/hello_world#deploy-to-STM32F746

    主机操作系统: ubuntu20
    开发板: STM32f746 NG DISCO

    pip3 install --user mbed-cli mercurial

    设置 gnu gcc 工具链:
        mbed config -G GCC_ARM $HOME/programs/gcc-arm-none-eabi-9-2020-q2-update/bin

    mbed config --list

    下载TensorFlow源码:
        git clone https://github.com/tensorflow/tensorflow.git

    生成basic的项目:
        cd tensorflow
        make -f tensorflow/lite/micro/tools/make/Makefile TARGET=mbed TAGS="CMSIS disco_f746ng" generate_hello_world_mbed_project

    会生成目录 mbed:
        cd tensorflow/lite/micro/tools/make/gen/mbed_cortex-m4/prj/hello_world/mbed

    mbed环境:
        mbed config root .
        mbed deploy

        pip3 install -r mbed-os/requirements.txt --user

        执行下面的snippet, 修改默认的C++98到C++11
        python -c 'import fileinput, glob;
        for filename in glob.glob("mbed-os/tools/profiles/*.json"):
            for line in fileinput.input(filename, inplace=True):
                print line.replace("\"-std=gnu++98\"","\"-std=c++11\", \"-fpermissive\"")'

        mbed compile -m DISCO_F746NG -t GCC_ARM

        生成文件:
            ./BUILD/DISCO_F746NG/GCC_ARM/mbed.bin

        把这个文件烧写到 DISCO_F746NG 上就可以了

    我使用的安装stlink工具, https://github.com/stlink-org/stlink/releases
    包括: st-flash st-info st-util
        sudo apt install stlink-tools
    
    烧写:
        st-flash write ./BUILD/DISCO_F746NG/GCC_ARM/mbed.bin 0x8000000
