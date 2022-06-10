use std::collections::HashMap;

use convert_case::{Case, Casing};
use serde_json::{json, Value};
use tera::{Context, Result, Tera};

fn main() {
    let context = Context::from_value(json!({ "users": {
        "name0": [
            "xiaoming0",
            "xiaoming1"
        ],
        "name1": "xiaoming",
        "name2": {
            "age": 12,
        }
    }}))
    .unwrap();

    let mut tera = Tera::default();

    tera.register_function("pascal", |args: &HashMap<String, Value>| -> Result<Value> {
        let text = args.get("text").unwrap().as_str().unwrap();
        Ok(text.to_case(Case::Pascal).into())
    });

    println!("{:?}", &tera);

    let ret = tera
        .render_str(include_str!("./test.tera"), &context)
        .unwrap();
    println!("{}", &ret);
}
