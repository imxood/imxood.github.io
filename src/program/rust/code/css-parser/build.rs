use std::{collections::HashMap, process::Command};

use convert_case::{Case, Casing};
use serde_json::{json, Value};
use simplelog::*;
use tera::{Context, Result, Tera};

fn build_properties() {
    let current_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let in_file = trace_file(current_dir.clone() + "/src/hbs/properties.tera");
    let out_file = current_dir.clone() + "/src/properties.rs";

    let data = json!({
        "properties": {
            "width": "Float",
            "height": "Float",

            "background": {
                "background-color": "Color",
                "background-position-x": "Float",
                "background-position-y": "Float",
            },

            // "display": {
            //     "flex": [
            //         "flex-direction",
            //         "flex-wrap",
            //         "flex-flow",
            //         "align-items",
            //         "align-content",
            //         "justify-content",
            //     ],
            //     "flex-item": [
            //         "flex",
            //         "flex-gow",
            //         "flex-shrink",
            //         "flex-basis",
            //         "align-self",
            //         "order",
            //     ],
            //     "grid": [
            //         /* grid-gap: <grid-row-gap> <grid-column-gap>; */
            //         {
            //             "grid-gap": [
            //                 "grid-row-gap",
            //                 "grid-column-gap",
            //             ]
            //         },
            //         /* place-items: <align-items> <justify-items>; */
            //         {
            //             "place-items": [
            //                 "align-items",
            //                 "justify-items",
            //             ]
            //         },
            //         /* place-content: <align-content> <justify-content>; */
            //         {
            //             "place-content": [
            //                 "align-content",
            //                 "justify-content",
            //             ]
            //         },
            //         /* grid-template: <grid-template-columns> <grid-template-rows> <grid-template-areas>; */
            //         {
            //             "grid-template": [
            //                 "grid-template-columns",
            //                 "grid-template-rows",
            //                 "grid-template-areas",
            //             ]
            //         },
            //         /* grid: <grid-template-rows> <grid-template-columns> <grid-template-areas> <grid-auto-rows> <grid-auto-columns> <grid-auto-flow>; */
            //         {
            //             "grid": [
            //                 "grid-auto-rows",
            //                 "grid-auto-columns",
            //                 "grid-auto-flow",
            //             ]
            //         }
            //     ],
            //     "grid-item": [
            //         "flex",
            //         "flex-gow",
            //         "flex-shrink",
            //         "flex-basis",
            //         "align-self",
            //         "order",
            //     ],
            // },
        }
    });

    let context = Context::from_value(data).unwrap();

    let mut tera = Tera::default();

    tera.register_function("pascal", |args: &HashMap<String, Value>| -> Result<Value> {
        let text = args.get("text").unwrap().as_str().unwrap();
        Ok(text.to_case(Case::Pascal).into())
    });
    tera.register_function("snake", |args: &HashMap<String, Value>| -> Result<Value> {
        let text = args.get("text").unwrap().as_str().unwrap();
        Ok(text.to_case(Case::Snake).into())
    });
    tera.register_function("kebab", |args: &HashMap<String, Value>| -> Result<Value> {
        let text = args.get("text").unwrap().as_str().unwrap();
        Ok(text.to_case(Case::Kebab).into())
    });

    tera.add_template_file(&in_file, Some("properties"))
        .unwrap();

    tera.render_to(
        "properties",
        &context,
        std::fs::File::create(&out_file).unwrap(),
    )
    .unwrap();

    rustfmt_generated_file(&out_file);
}

fn rustfmt_generated_file(source_file: &str) {
    let rustfmt_path = which::which("rustfmt")
        .expect("cann't find rustfmt, please install \"rustup component add rustfmt\"");
    Command::new(&*rustfmt_path)
        .arg(source_file)
        .output()
        .expect(&format!(
            "rustfmt format file failed, file: {}",
            source_file
        ));
}

fn trace_file(file: String) -> String {
    println!("cargo:rerun-if-changed={}", &file);
    file
}

fn main() {
    log_init();

    println!("cargo:rerun-if-changed=build.rs");

    build_properties();

    log::info!("build ok");
}

fn log_init() {
    let mut builder = ConfigBuilder::new();
    builder.set_time_offset_to_local().unwrap();
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        builder.build(),
        std::fs::File::create("build.log").unwrap(),
    )])
    .unwrap();
}
