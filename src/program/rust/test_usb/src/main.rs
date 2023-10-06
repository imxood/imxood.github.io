use std::time::Duration;

extern crate libusb;

fn main() {
    let context = libusb::Context::new().unwrap();

    for device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if device_desc.vendor_id() == 0x1a86 && device_desc.product_id() == 0x5537 {
            println!(
                "Bus {:03} Device {:03} ID 0x{:04x}:0x{:04x}",
                device.bus_number(),
                device.address(),
                device_desc.vendor_id(),
                device_desc.product_id()
            );
            // 获取配置描述符的数量
            let num_configs = device_desc.num_configurations();
            println!("num_configs: {}", num_configs);
            for idx_config in 0..num_configs {
                println!("\t idx_config: {}", idx_config);
                // 根据 配置描述符索引 获取 配置描述符
                let config_desc = device.config_descriptor(idx_config).unwrap();
                // 获取 配置描述符 中 接口描述符 的数量
                let num_iface = config_desc.num_interfaces();
                println!("\t num_iface: {}", num_iface);
                // 遍历 配置描述符 中的所有 接口描述符
                for iface in config_desc.interfaces() {
                    for endp in iface.descriptors() {
                        let num_endp = endp.num_endpoints();
                        println!("\t\t num_endp: {}", num_endp);
                    }
                }
            }

            // 读取 激活的 配置描述符
            let config_desc = device.active_config_descriptor().unwrap();
            // 获取 激活的配置描述符 中 接口描述符 的数量
            let num_iface = config_desc.num_interfaces();
            println!("num_iface: {}", num_iface);
            // 遍历 激活的配置描述符 中的所有 接口描述符
            for iface in config_desc.interfaces() {
                for endp in iface.descriptors() {
                    let num_endp = endp.num_endpoints();
                    println!("\t\t num_endp: {}", num_endp);
                    for endp_desc in endp.endpoint_descriptors() {
                        println!("\t\t\t endp addr: 0x{:x}", endp_desc.address());
                        println!(
                            "\t\t\t endp transfer type: 0x{:?}",
                            endp_desc.transfer_type()
                        );
                        println!("\t\t\t endp transfer dir: {:?}", endp_desc.direction());
                        println!(
                            "\t\t\t endp transfer max package size: 0x{:x}",
                            endp_desc.max_packet_size()
                        );
                    }
                }
            }

            let mut handle = device.open().unwrap();
            println!("Device Opened");

            let iface = 0;

            let ret = handle.kernel_driver_active(iface).unwrap();
            if ret {
                println!("Kernel Driver Active");
                handle.detach_kernel_driver(iface).unwrap();
                println!("Kernel Driver Detached");
            }
            handle.claim_interface(iface).unwrap();
            println!("Claim Interface!");

            let endpoint = 0x02;
            let buf = [0x01; 64];

            let size = handle
                .write_interrupt(endpoint, &buf[..], Duration::from_secs(3))
                .unwrap();
            println!("Write successful, write size: {}", size);
            println!("Handle closed")
        }
    }
}
