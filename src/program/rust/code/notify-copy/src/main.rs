#![feature(absolute_path)]

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::RecvTimeoutError;
use std::time::Duration;

use clap::ArgGroup;
use notify::Event;
use notify::EventKind;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// 要监控的源文件, 文件或目录, 可以设置多个
    #[clap(short, long, multiple_values(true))]
    sources: Vec<String>,

    /// 复制源文件 到 这个参数指定的目录
    #[clap(short, long)]
    target_dir: String,
}

fn main() {
    // std::env::set_current_dir(&work_dir).unwrap();

    let cli = Cli::parse();

    for source in &cli.sources {
        println!("{}", source);
    }

    println!("work_dir: {:?}", std::env::current_dir().unwrap());

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx).unwrap();

    for source in &cli.sources {
        watcher
            .watch(Path::new(source), RecursiveMode::Recursive)
            .unwrap();
    }

    let mut file_changed = false;

    loop {
        let e = rx.recv();
        match e {
            Ok(Ok(Event {
                kind: EventKind::Modify(modify),
                paths,
                attrs,
            })) => {
                if let Some(filename) = paths.get(0) {
                    println!("{}", filename.to_str().unwrap());
                    // let abs_filepath = std::path::absolute(&log_file).unwrap();
                    // println!("{:?} 文件已修改", filename);
                    // if filename == &abs_filepath {
                    //     file_changed = true;
                    //     break;
                    // }
                }
                println!(
                    "watch event -- modify: {:?}, paths: {:?}, attrs: {:?}",
                    modify, paths, attrs
                );
            }
            Ok(e) => {
                println!("ignore watch event -- {:?}", e);
            }
            // Err(RecvTimeoutError::Timeout) => {
            //     println!("watch error -- timeout");
            //     break;
            // }
            // Err(RecvTimeoutError::Disconnected) => {
            //     println!("watch error -- {:?}", e);
            //     break;
            // }
            Err(e) => {
                println!("watch error -- {:?}", e);
                break;
            }
        }
    }

    // if file_changed {
    //     let logfile = File::open(&log_file).expect("Error in reading file");
    //     let reader = BufReader::new(logfile);
    //     for line in reader.lines() {
    //         println!("{}", line.unwrap());
    //     }
    // }
}
