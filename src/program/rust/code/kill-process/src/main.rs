use sysinfo::{PidExt, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};

use structopt::StructOpt;

/// Kill a process list
///
/// if run success, will output "run success"
///
/// if run failed, will output "run failed"
///
#[derive(StructOpt, Debug)]
#[structopt(name = "kill-process")]
struct Opt {
    /// Specifies a list of process names
    #[structopt(short, long)]
    names: Vec<String>,

    /// Specifies a list of process ids
    #[structopt(short, long)]
    pids: Vec<u32>,
}

#[derive(Debug, Default)]
struct ProcessList {
    pub pids: Vec<u32>,
    pub names: Vec<String>,
}

fn main() -> Result<(), String> {
    let mut opt = Opt::from_args();
    let mut process_list = ProcessList::default();

    process_list.names.append(&mut opt.names);
    process_list.pids.append(&mut opt.pids);

    let sys = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::new().with_user()),
    );

    if !kill_process(&sys, process_list) {
        return Err("run failed".to_string());
    }

    println!("run success");
    Ok(())
}

fn kill_process(sys: &System, mut process_list: ProcessList) -> bool {
    for (_, process) in sys.processes() {
        let pid = process.pid().as_u32();
        if let Some(idx) = process_list.pids.iter().position(|p| *p == pid) {
            if !process.kill() {
                println!("process pid `{pid}` kill failed");
                return false;
            }
            println!("process pid `{pid}` kill success");
            process_list.pids.remove(idx);
            continue;
        }

        let name = process.name().to_lowercase();
        if let Some(idx) = process_list
            .names
            .iter()
            .position(|n| &n.to_lowercase() == &name)
        {
            if !process.kill() {
                println!("process name `{name}` kill failed");
                return false;
            }
            println!("process name `{name}` kill success");
            process_list.names.remove(idx);
            continue;
        }
    }

    for pid in process_list.pids.iter() {
        println!("process pid `{pid}` not exist");
    }
    for name in process_list.names.iter() {
        println!("process name `{name}` not exist");
    }

    true
}
