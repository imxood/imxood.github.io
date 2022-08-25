use std::io::{self, Write};
use std::time::Duration;

fn main() {
    let port_name = "COM3";
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
    let mut buf_read: [u8; 1] = [0];
    loop {
        match port.read_exact(&mut buf_read) {
            Ok(_) => {
                print!("{}", buf_read[0] as char);
                std::io::stdout().flush().unwrap();
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
