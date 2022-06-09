use simplelog::*;

fn main() {
    log_init();
    log::info!("hello");
}

fn log_init() {
    let current_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut builder = ConfigBuilder::new();
    builder.set_time_offset_to_local().unwrap();
    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Info,
            builder.build(),
            std::fs::File::create(current_dir + "/build.log").unwrap(),
        ),
        TermLogger::new(
            LevelFilter::Info,
            builder.build(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
    ])
    .unwrap();
}
