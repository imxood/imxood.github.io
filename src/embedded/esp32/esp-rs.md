cargo install cargo-espflash

git clone https://github.com/esp-rs/esp-wifi.git

cd esp-wifi/examples-esp32c3

cargo build --features "wifi,embedded-svc"

cargo espflash --features "wifi,embedded-svc" --example dhcp --speed 2000000
