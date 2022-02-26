## 环境

## simulator on PC

https://docs.lvgl.io/v7/en/html/get-started/pc-simulator.html

1. 下载 Eclipse CDT IDE 环境
    
    https://github.com/gnu-mcu-eclipse/org.eclipse.epp.packages/releases/

2. 安装 SDL2

    On Linux:

        sudo apt-get update && sudo apt-get install -y build-essential libsdl2-dev

3. 下载 eclipse's simulator project:

    git clone --recursive https://github.com/lvgl/lv_sim_eclipse_sdl.git

4. 使用cmake编译这个项目

    cd lv_sim_eclipse_sdl
    mkdir build
    cd build
    cmake ..
    make -j

4. 打开 eclipse CDT, 导入 simulator project

5. 在 "Run Configurations..." 配置编译 "C/C++ Application"

    ![](lvgl/2022-02-26-13-01-10.png)

    编译命令是:

        make -j -C ${workspace_project_locations}/build

    这样就可以编译并运行了

## 例子

    f746 disco:

        git clone --recursive https://github.com/lvgl/lv-port_stm32f746_disco.git

