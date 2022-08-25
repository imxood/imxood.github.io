use std::io::{self, Write};
use std::str;
use std::time::Duration;

fn main() {
    let port_name = "COM15";
    let baud_rate = 115200;

    let builder = serialport::new(port_name, baud_rate);
    println!("{:?}", &builder);
    let mut port = builder
        .timeout(Duration::from_millis(10))
        .open()
        .unwrap_or_else(|e| {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        });

    println!("port '{port_name}' baud_rate {baud_rate}");
    let buf = [27];
    let mut buf_read: [u8; 256] = [0; 256];

    let len = port.write("rf_single_tone 1000000\r\n".as_bytes()).unwrap();
    println!("len: {}", len);

    loop {
        match port.read(&mut buf_read) {
            Ok(len) => {
                print!(
                    "len: {}, {:?}",
                    len,
                    str::from_utf8(&buf_read[0..len]).unwrap()
                );
                // std::io::stdout().flush().unwrap();
            }
            Err(e) => {
                if e.kind() != io::ErrorKind::TimedOut {
                    eprintln!("{:?}", e);
                }
                for _ in 0..3 {
                    match port.write(&buf) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("{:?}", e);
                            break;
                        }
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
            }
        }
    }
}
