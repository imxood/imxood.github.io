#![feature(absolute_path)]

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::sync::mpsc::RecvTimeoutError;
use std::time::Duration;

use notify::Event;
use notify::EventKind;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;

fn main() {
    // std::env::set_current_dir(&work_dir).unwrap();

    println!("current_dir: {:?}", std::env::current_dir().unwrap());

    let log_file = PathBuf::from("test.log");

    // 如果文件存在 则清空文件内容, 否则 创建新文件
    File::create(&log_file).unwrap();

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx).unwrap();

    watcher.watch(&log_file, RecursiveMode::Recursive).unwrap();

    let mut file_changed = false;

    loop {
        let e = rx.recv_timeout(Duration::from_secs(30));
        match e {
            Ok(Ok(Event {
                kind: EventKind::Modify(modify),
                paths,
                attrs,
            })) => {
                if let Some(filename) = paths.get(0) {
                    let abs_filepath = std::path::absolute(&log_file).unwrap();
                    println!("{:?} 文件已修改", filename);
                    if filename == &abs_filepath {
                        file_changed = true;
                        break;
                    }
                }
                println!(
                    "watch event -- modify: {:?}, paths: {:?}, attrs: {:?}",
                    modify, paths, attrs
                );
            }
            Ok(e) => {
                println!("ignore watch event -- {:?}", e);
            }
            Err(RecvTimeoutError::Timeout) => {
                println!("watch error -- timeout");
                break;
            }
            Err(RecvTimeoutError::Disconnected) => {
                println!("watch error -- {:?}", e);
                break;
            }
        }
    }

    if file_changed {
        let logfile = File::open(&log_file).expect("Error in reading file");
        let reader = BufReader::new(logfile);
        for line in reader.lines() {
            println!("{}", line.unwrap());
        }
    }
}
