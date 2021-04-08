# Rust 笔记

## Rust 的 安装与卸载

    参考链接:
        https://www.rust-lang.org/zh-CN/tools/install

    安装:
        curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

    版本号:
        rustc --version

    升级:
        rustup update stable
        rustup update

    设置默认 版本:
        rustup default stable
        rustup default nightly

    工具链
        rustup toolchain install nightly
        rustup toolchain install nightly-2020-11-19
        rustup toolchain list
        rustup default nightly-2020-11-19

        ps:
            "2020-11-19" 这个时间是在 rust 的 git 中的tag上找的

        rustup override set nightly

	## riscv
    rustup target add riscv32imac-unknown-none-elf

    卸载:
        rustup self uninstall

## cargo 命令

    添加工具:
        cargo install cargo-generate
        cargo install cargo-edit
        cargo install mdbook

    创建项目:
        cargo new hello_cargo

    编译 并生成可执行程序:
        cd hello_cargo
        cargo build
        cargo build --release

    编译 但不生成 可执行程序:
        cargo check

    运行目标程序 (也可以一步构建项目):
        cargo run

    创建:
        cargo new hello_world <--bin>, 创建一个二进制程序
        cargo new hello_world --lib, 创建一个库


## vscode 中 rust 的插件

    1. rust-analyzer
    2. Crates
    3. Better TOML
    4. CodeLLDB

## Rust hello_world

    编写点一个程序:
        vim main.rs

        fn main() {
            println!("Hello, world!");
        }

    编译:
        rustc main.rs

    运行:
        ./main

## 库

    kurbo
        曲线工具

    lyon
        路径细分库, 可以用于基于GPU的2D图形渲染

    tokio
        https://github.com/tokio-rs/tokio

    embedded_graphics
        mcu gui

    embedded_sdmmc

### iced

    sudo apt install libssl-dev

    官方的例子:

        git clone https://github.com/hecrj/iced.git

        cd iced

        cargo build
        cargo build -vv 显示详细的编译命令

        // --features glow,glow_canvas
        cargo run --package todos

    // 如果无法运行, 缺少显示驱动
    sudo apt install libvulkan1 mesa-vulkan-drivers vulkan-utils

### iced: 编译 并 运行所有的 examples

    #!/bin/bash

    examples=(iced_core iced_futures iced_graphics iced_native iced_style iced_glow iced_glutin iced_winit iced_web iced_wgpu bezier_tool iced clock color_palette counter custom_widget download_progress events game_of_life geometry integration pane_grid pick_list pokedex progress_bar qr_code scrollable solar_system stopwatch styling svg todos tour)

    for example in \${examples[@]}; do
        cargo build --verbose --package $example
    done

    for example in \${examples[@]}; do
        echo "start run: cargo run --verbose --package $example"
        cargo run --verbose --package $example
    done


### iced 例子

    学习基本的布局:
        target/debug/pane_grid
        examples/tour

    学习canvas:
        examples/clock


## libusb, usb库

    在windows上找不到libusb库, 在 ~/.cargo/config 中添加:

    [target.x86_64-pc-windows-msvc.'usb-1.0']
    rustc-link-search = ['D:\libs\64bit']
    rustc-link-lib = ['libusb-1.0']

## wasm 开发

    安装 wasm-pack:

        curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

    cargo install cargo-generate

    cargo generate --git https://github.com/rustwasm/wasm-pack-template -n wasm-app

    cd wasm-app
    wasm-pack build

    npm init wasm-app www

    cd www
    yarn

    cd ../pkg
    yarn link

    cd ../www
    yarn link "wasm-app"


## vue vite rust wasm

    yarn create @vitejs/app my-vue-app --template vue

    cd my-vue-app
    yarn add -D vite-plugin-rsw

    yarn

    cargo generate --git https://github.com/rustwasm/wasm-pack-template -n wasm-app

    vite.config.js:

        import { defineConfig } from "vite";
        import vue from "@vitejs/plugin-vue";
        import ViteRsw from 'vite-plugin-rsw';

        // https://vitejs.dev/config/
        export default defineConfig({
            plugins: [
                vue(),
                ViteRsw({
                    mode: "release",
                    crates: ["wasm-app"],
                }),
            ],
        });
