编译

https://github.com/rustdesk/rustdesk#raw-steps-to-build

windows系统

vcpkg install libvpx:x64-windows-static libyuv:x64-windows-static opus:x64-windows-static aom:x64-windows-static

cargo build --release

cargo run --release
