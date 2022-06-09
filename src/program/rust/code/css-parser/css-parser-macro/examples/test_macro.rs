use css_parser_macro::define_property_list;

fn main() {}

define_property_list!(
    [width, "<length> | <percentage>", "enum"],
    [height, "<length> | <percentage>", "enum"],
);
