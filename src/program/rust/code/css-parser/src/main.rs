use simplelog::*;

use crate::selectors::parse_css;

// use crate::build::build_properties;

mod color;
mod display;
mod parse;
mod properties;
mod selectors;
// mod build;
mod rule;
mod serialize;
mod types;

fn main() {
    log_init();

    let css_data = include_str!("./simple.css");
    let (_, ret) = parse_css(css_data).unwrap();
    println!("{:#?}", &ret);

    log::info!("build ok");
}

fn log_init() {
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
