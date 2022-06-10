use simplelog::*;

pub fn log_init() {
    let mut builder = ConfigBuilder::new();
    builder.set_time_offset_to_local().unwrap();
    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Info,
            builder.build(),
            std::fs::File::create("build.log").unwrap(),
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
