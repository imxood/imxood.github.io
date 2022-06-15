use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/css.pest"]
pub struct CssParser;

#[test]
fn test_css() {}
