use pest_examples::properties::entry::Entities;
use pest_examples::properties::*;

fn main() {
    let css = Entities::parse_str(include_str!("./simple.css"));
    println!("{}", css.unwrap().to_css_string());
}
