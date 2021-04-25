# Rust嵌入式开发环境

## vscode 开发环境, 硬件: stm32f746g-disco

    安装必要的工具:

        cargo install cargo-generate
        rustup target add thumbv7em-none-eabihf

        rustup component add llvm-tools-preview
        cargo install cargo-binutils

    生成项目:
        cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart

        cd <DIR>

    重命名 .cargo/config 到 .cargo/config.toml (不改也可以, 都支持, 改了有高亮显示):

        [build]
        target = "thumbv7em-none-eabihf"     # Cortex-M4F and Cortex-M7F (with FPU)

    修改 memory.x 中的 FLASH 和 RAM 的源地址及大小:
        FLASH : ORIGIN = 0x08000000, LENGTH = 1M
        RAM : ORIGIN = 0x20000000, LENGTH = 256K

    更新一下 Cargo.toml 中依赖的版本.

    编译:
        cargo build

    查看大小
        cargo size --bin app
        cargo size --bin app -- -A

    查看二进制信息:
        cargo readobj --bin app -- -file-headers

    反汇编二进制文件:
        cargo objdump --bin app --release -- --disassemble --no-show-raw-insn --print-imm-hex

    调试, 在launch文件中添加:
        {
			/* Configuration for the STM32F303 Discovery board */
			"type": "cortex-debug",
			"request": "launch",
			"name": "Debug (OpenOCD)",
			"servertype": "openocd",
			"cwd": "${workspaceRoot}",
			"preLaunchTask": "Cargo Build (debug)",
			"runToMain": true,
			"executable": "./target/thumbv7em-none-eabihf/debug/app",
			"configFiles": ["interface/stlink.cfg", "board/stm32f746g-disco.cfg"],
		}

## 优化

官方的文档写的很清晰了

[The Embedded Rust Book - Optimizations: the speed size tradeoff](https://docs.rust-embedded.org/book/unsorted/speed-vs-size.html)
