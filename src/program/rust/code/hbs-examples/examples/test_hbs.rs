use std::error::Error;

use handlebars::{handlebars_helper, Handlebars, JsonValue};
use serde_json::json;

handlebars_helper!(is_array: |target: JsonValue| target.is_array());
handlebars_helper!(is_object: |target: JsonValue| target.is_object());

fn main() -> Result<(), Box<dyn Error>> {
    let mut reg = Handlebars::new();

    // {{#if (not (is_array sub_type_name))}} 使用括号 来使用注册函数, 可以写多层
    reg.register_helper("is_array", Box::new(is_array));
    reg.register_helper("is_object", Box::new(is_object));

    println!("reg: {:?}", &reg);

    let raw = r#"
    {{#each users}}
        {{@first}} {{@key}}: {{this}} {{@last}}
       
        {{#if @first}}
            测试 使用索引访问数组 {{this.[1]}}
        {{/if}}

    {{/each}}

    {{#with users}}
        测试 with
            {{name1}}

        测试 eq
            {{#if (eq name1 "xiaoming")}}
                name1 等于 "xiaoming"
            {{else}}
                name1 不等于 "xiaoming"
            {{/if}}
    {{/with}}

    {{#each users as |value key| }}
        {{key}}
        
        {{#if (is_array value)}}
            true {{value}}
        {{else}}
            false
        {{/if}}

        {{#unless (is_array value)}}
            unless, 不是数组 {{value}}
        {{/unless}}

        {{#if (and (not (is_array value)) (not (is_object value)))}}
            if (and (not (is_array value)) (not (is_object value))) -- 既不是数组 也不是对象, {{key}}: {{value}}
        {{/if}}
    {{/each}}

    "#;
    println!(
        "{}",
        reg.render_template(
            raw,
            &json!({"users": {
                "name0": [
                    "xiaoming0",
                    "xiaoming1"
                ],
                "name1": "xiaoming",
                "name2": {
                    "age": 12,
                }
            },})
        )?
    );

    Ok(())
}
