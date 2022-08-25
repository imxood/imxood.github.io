#![feature(panic_info_message)]
#![feature(buf_read_has_data_left)]

use std::time::Duration;

use custom::{mc19_burning, mc19_reboot, rf_single_tone, serial_monitor, test_io};
use step::{AtDevice, Step};

use clap::Parser;

mod custom;
mod error;
mod step;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// 被测模块
    #[clap(short, long)]
    target: String,

    /// 烧写测试固件
    #[clap(short, long)]
    burning: bool,

    /// 重启
    #[clap(short, long)]
    reboot: bool,

    /// IO测试
    #[clap(short, long)]
    io: bool,

    /// 发射单音信号
    #[clap(short, long)]
    single_tone: bool,

    /// 串口监视器
    #[clap(short, long)]
    monitor: bool,

    /// 工作模式
    #[clap(short, long)]
    work: bool,

    /// workspace目录
    #[clap(long, default_value_t = String::from(r"E:\develop\genie-bt-mesh-stack-mc19-mp\EH-MC19-TestCase"))]
    work_dir: String,

    #[clap(long)]
    fw: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    std::env::set_current_dir(&cli.work_dir);

    let test_module_port_name = "COM15";
    let test_module_baudrate = 115200;

    let test_board_port_name = "COM16";
    let test_board_baudrate = 115200;

    let mut test_module = AtDevice::new(test_module_port_name, test_module_baudrate, "测试模块")
        .expect(&format!("访问端口{}", test_module_port_name));

    let mut test_board = AtDevice::new(test_board_port_name, test_board_baudrate, "测试板")
        .expect(&format!("访问端口{}", test_board_port_name));

    let mut fw = "E:/develop/eh-mc19/ehong-test/EH-MC19-TestCase/tool/total_image.hexf";

    if &cli.target == "tmy" {
        if cli.burning {
            mc19_burning(&mut test_board, &mut test_module, fw);
        }

        if cli.reboot {
            mc19_reboot(&mut test_board, &mut test_module);
        }
    }

    if &cli.target == "mc19" {
        let fw = if let Some(fw) = &cli.fw {
            fw
        } else {
            r"fw\total_image.hexf"
        };
        if cli.burning {
            mc19_burning(&mut test_board, &mut test_module, fw);
        }

        if cli.reboot {
            mc19_reboot(&mut test_board, &mut test_module);
        }

        if cli.io {
            test_io(&mut test_module);
        }

        if cli.single_tone {
            rf_single_tone(&mut test_module);
        }

        if cli.single_tone {
            loop {
                rf_single_tone(&mut test_module);
            }
        }

        if cli.monitor {
            serial_monitor(&mut test_module);
        }
    }
}
