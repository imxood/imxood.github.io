use css_parser::selectors::parse_css;
use css_parser::utils::log_init;

fn main() {
    log_init();
    let css_data = include_str!("./simple.css");
    let (_, ret) = parse_css(css_data).unwrap();
    println!("{:#?}", &ret);
}
