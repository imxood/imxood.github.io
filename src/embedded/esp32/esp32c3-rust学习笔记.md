# risc-v 芯片 esp32c3

## esp-idf-sys native方式编译 分析

    1. native编译

        1> 自动下载 esp-idf cmake ninja riscv-esp工具链

        2> 使用 cmake 编译 esp-idf， 配置文件是： sdkconfig 和 sdkconfig_defaults

            把 esp-idf-sys 中的 resources/cmake_project 作为一个基本的项目, 复制到你的项目编译输出路径下:
                target/riscv32imc-esp-espidf/debug/build/esp-idf-sys-eac6c6bb186a6e4d/out
                    .
                    ├── build
                    ├── CMakeLists.txt
                    └── main.c

                sdkconfig 和 sdkconfig_defaults 是 从环境变量 ESP_IDF_SDKCONFIG 和 ESP_IDF_SDKCONFIG_DEFAULTS 中取到的

            主要过程是：
              cmake::Config::new(&out_dir)
                .generator("Ninja")
                .out_dir(&out_dir)
                .no_build_target(true)
                .define("CMAKE_TOOLCHAIN_FILE", &cmake_toolchain_file)
                .always_configure(true)
                .pic(false)
                .asmflag(asm_flags)
                .cflag(c_flags)
                .cxxflag(cxx_flags)
                .env("IDF_PATH", &idf.esp_idf.worktree())
                .env("PATH", &idf.exported_path)
                .env("SDKCONFIG", sdkconfig)
                .env("SDKCONFIG_DEFAULTS", sdkconfig_defaults)
                .env("IDF_TARGET", &chip.to_string())
                .build();

        3>. 解析 cmake 的输出结果, 如: link_args sdkconfig_json, 把解析的结果返回 -- EspIdfBuildOutput

    2. 生成rs代码

        一个bindgen需要的头文件 header_file:
            src/include/esp-idf/bindings.h

        执行, 返回值是 bindings_file:
            bindgen::run(
                build_output
                    .bindgen
                    .builder()?
                    .ctypes_prefix("c_types")
                    .header(header_file.try_to_str()?)
                    .blacklist_function("strtold")
                    .blacklist_function("_strtold_r")
                    .clang_args(build_output.components.clang_args())
                    .clang_args(vec![
                        "-target",
                        if mcu == "esp32c3" {
                            // Necessary to pass explicitly, because of https://github.com/rust-lang/rust-bindgen/issues/1555
                            "riscv32"
                        } else {
                            // We don't really have a similar issue with Xtensa, but we pass it explicitly as well just in case
                            "xtensa"
                        },
                    ]),
            )?;

        bindings_file 这里是: target/riscv32imc-esp-espidf/debug/build/esp-idf-sys-e842de6eba130a3d/out/bindings.rs
        这是 头文件中声明的 rust绑定. 同时, 这中间使用的各种 args 也会以 println!("cargo: ... "): 的形式输出

    3. 在 esp-idf-sys 中刚好有 src/lib.rs:
        
        include!(env!("EMBUILD_GENERATED_BINDINGS_FILE"));

    上面几个过程就有了 esp32 的 c bind 的 crate 了.