pub fn kill_old_process(&self) {
    let program_name = Path::new(self.command.get_program().to_str().expect("OsStr异常"))
        .file_name()
        .expect("OsStr异常")
        .to_str()
        .expect("OsStr异常");

    let sys = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::new().with_user()),
    );

    for (_, process) in sys.processes() {
        if process.name() == program_name {
            if process.kill() {
                println!("发送 kill 信号成功");
            } else {
                println!("发送 kill 失败");
            }
        }
    }
}
