# libusb 示例

    vcpkg install libusb --triplet=x64-windows

    $env:VCPKGRS_TRIPLET="x64-windows"
    $env:VCPKGRS_DYNAMIC=1

    git clone https://github.com/a1ien/rusb.git

    cd rusb

    cargo run --example list_devices
