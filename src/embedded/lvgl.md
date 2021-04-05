## 环境

## simulator on PC

    https://docs.lvgl.io/v7/en/html/get-started/pc-simulator.html

    1. 需要 Eclipse CDT IDE 环境
        
        https://github.com/gnu-mcu-eclipse/org.eclipse.epp.packages/releases/

    2. 安装 SDL2

        On Linux:

            sudo apt-get update && sudo apt-get install -y build-essential libsdl2-dev

    3. 下载 eclipse's simulator project:

        git clone --recursive https://github.com/littlevgl/pc_simulator_sdl_eclipse.git

    4. 把 simulator project 导入 eclipse CDT 中, 编译 并 运行 Debug/lv_sim_eclipse_sdl

## 例子

    f746 disco:

        git clone --recursive https://github.com/lvgl/lv-port_stm32f746_disco.git
