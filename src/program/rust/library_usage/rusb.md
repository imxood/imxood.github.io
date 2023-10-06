# libusb 示例

    vcpkg install libusb --triplet=x64-windows

    vcpkg install libusb --triplet=x86-windows

    $env:VCPKGRS_TRIPLET="x64-windows"
    $env:VCPKGRS_DYNAMIC=1

    git clone https://github.com/a1ien/rusb.git

    需要复制两个文件:
        vcpkg\installed\x64-windows\bin\libusb-1.0.dll
        vcpkg\installed\x64-windows\lib\libusb-1.0.lib
    到项目根路径下.

    cd rusb
    cargo run --example read_device

    std::env::set_var("VCPKGRS_TRIPLET", "x64-windows");
    std::env::set_var("VCPKGRS_DYNAMIC", "1");

    // std::env::set_var("TARGET", "x86_64-pc-windows-msvc");
    // std::env::set_var("HOST", "x64-windows-static");
    // std::env::set_var("OPT_LEVEL", "1");
    // std::env::set_var("CARGO_CFG_TARGET_FEATURE", "crt-static");

    // std::env::set_var("VCPKGRS_TRIPLET", "x64-windows-static-md");