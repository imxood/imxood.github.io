use simplelog::*;

#[path = "src/color.rs"]
mod color;
#[path = "src/parse.rs"]
mod parse;
#[path = "src/rule.rs"]
mod rule;
#[path = "src/serialize.rs"]
mod serialize;
#[path = "src/types.rs"]
mod types;

use rule::PropDescriptor;

fn main() {
    log_init();

    println!("cargo:rerun-if-changed=build.rs");

    build_properties();

    log::info!("build ok");
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

use std::{collections::HashMap, process::Command};

use convert_case::{Case, Casing};
use serde_json::{json, Value};
use tera::{Context, Result, Tera};

pub fn build_properties() {
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
    let prop_array = json!([
        {
            "name": "width",
            "desc": "<length> | <percentage>",
            "type": "enum",
            "list": [
                "length", "percentage"
            ]
        },
        {
            "name": "height",
            "desc": "<length> | <percentage>",
            "type": "enum",
            "list": [
                "length", "percentage"
            ]
        },
        /* background */
        {
            "name": "background",
            "desc": "[<background-color> | <background-image>] <background-position>?",
            "type": "struct",
            "list": [
                "background-color", "background-image", "background-position"
            ]
        },
        {
            "name": "background-color",
            "desc": "<color>",
            "type": "struct",
            "list": [
                "color"
            ]
        },
        {
            "name": "background-image",
            "desc": "<image>",
            "type": "struct",
            "list": [
                "image"
            ]
        },
        {
            "name": "background-position",
            "desc": "<position>",
            "type": "struct",
            "list": [
                "position"
            ]
        },
        /* grid */
        {
            "name": "grid-template-columns",
            "desc": "<position>",
            "type": "struct",
            "list": [
                "position"
            ]
        },
        // {
        //     "name": "grid-template-rows",
        //     "desc": "<position>",
        //     "type": "struct",
        //     "list": [
        //         "position"
        //     ]
        // },
    ]);

    let current_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let properties_in_file = trace_file(current_dir.clone() + "/src/hbs/properties.tera");
    let properties_out_file = current_dir.clone() + "/src/properties.rs";

    let prop_desc = PropDescriptor::new(&prop_array);
    let struct_infos = prop_desc.struct_infos();
    log::info!("prop_desc.struct_infos(): {:#?}", &struct_infos);
    log::info!("prop_desc.parser(): {:#?}", &prop_desc.parser());

    log::info!("{:?}", "option(background-color)".to_case(Case::UpperCamel));

    let mut context = Context::new();
    context.insert("struct_infos", &struct_infos);

    let mut tera = init_tera();

    tera.add_template_file(&properties_in_file, Some("properties"))
        .unwrap();

    tera.render_to(
        "properties",
        &context,
        std::fs::File::create(&properties_out_file).unwrap(),
    )
    .unwrap();

    rustfmt_generated_file(&properties_out_file);
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

fn init_tera() -> Tera {
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
    tera
}

fn trace_file(file: String) -> String {
    println!("cargo:rerun-if-changed={}", &file);
    file
}
