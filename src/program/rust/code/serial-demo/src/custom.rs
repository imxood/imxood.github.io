use std::time::Duration;

use crate::step::{AtDevice, ExeDevice, RunDevice, Step};

pub fn check_boot_success(test_module: &mut AtDevice, ok: &str) {
    Step::new(&format!("检测 <{}> 是否重启成功", test_module.dev_name()))
        .with_readonly(true)
        .with_ok(ok)
        .with_timeout(Duration::from_secs(5))
        .run(test_module)
        .expect("at cmd run 异常");
}

pub fn reset_test_module(test_board: &mut AtDevice) {
    Step::new("复位模块")
        .with_cmd("AT+RST=02\r\n")
        .with_ok("reset ,OK")
        .with_timeout(Duration::from_secs(5))
        .run(test_board)
        .expect("at cmd run 异常");
}

pub fn mc19_reboot(test_board: &mut AtDevice, test_module: &mut AtDevice) {
    Step::new("拉低TIM")
        .with_cmd("AT+MC=00\r\n")
        .with_ok("OK")
        .with_timeout(Duration::from_secs(5))
        .run(test_board)
        .expect("at cmd run 异常");

    reset_test_module(test_board);
    // check_boot_success(test_module, "test-ready");
}

pub fn mc19_burning(test_board: &mut AtDevice, test_module: &mut AtDevice, fw_file: &str) {
    // Step::new("显示版本")
    //     .with_cmd("AT+VR\r\n")
    //     .with_ok("OK")
    //     .with_timeout(Duration::from_secs(5))
    //     .run(test_board)
    //     .expect("at cmd run 异常");

    Step::new("拉高TIM")
        .with_cmd("AT+MC=01\r\n")
        .with_ok("OK")
        .with_timeout(Duration::from_secs(5))
        .run(test_board)
        .expect("at cmd run 异常");

    // 复位后 进入 boot 模式
    reset_test_module(test_board);

    test_module.close();

    Step::new("烧写测试固件到被测模块")
        .with_ok("Write registers successfully!")
        .with_fail("Error: Device open failed,exiting the program")
        .with_timeout(Duration::from_secs(5))
        .run(
            &mut ExeDevice::new(
                r"tool\TG71XX_Programmer.exe",
                &vec![
                    "-P", "COM15", "-e", "1", "-u", "1500000", "-R", "1FFF4800", "-w", fw_file,
                ],
            )
            .expect("创建ExeDevice异常"),
        )
        .expect("子进程执行异常");
}

pub fn serial_monitor(test_module: &mut AtDevice) {
    Step::new("持续输出串口")
        .with_readonly(true)
        .with_endflag(true)
        .with_timeout(Duration::from_secs(1000))
        .run(test_module)
        .expect("at cmd run 异常");
}

pub fn test_io(test_module: &mut AtDevice) {
    Step::new("io测试")
        .with_cmd("test_io\n")
        .with_ok("cmd run success")
        .with_fail("cmd run failed")
        .with_timeout(Duration::from_secs(5))
        .run(test_module)
        .expect("at cmd run 异常");
}

pub fn rf_single_tone(test_module: &mut AtDevice) {
    Step::new("发射单音信号")
        .with_cmd("rf_single_tone 0x1f 0 1 4100000\n")
        .with_ok("cmd run success")
        .with_fail("cmd run failed")
        .with_timeout(Duration::from_secs(20))
        .run(test_module)
        .expect("at cmd run 异常");
}
