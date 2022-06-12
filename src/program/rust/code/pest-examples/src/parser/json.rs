use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/json.pest"]
pub struct JsonParser;

#[test]
fn test_json() {
    let pairs = JsonParser::parse(Rule::value, "[1, {}, null, true, \"a\"]")
        .unwrap_or_else(|e| panic!("{}", e));
    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::string => println!("String:  {}", inner_pair.as_str()),
                Rule::number => println!("Number:   {}", inner_pair.as_str()),
                Rule::object => println!("Object:   {}", inner_pair.as_str()),
                Rule::array => println!("Array:   {}", inner_pair.as_str()),
                Rule::boolean => println!("Boolean:   {}", inner_pair.as_str()),
                Rule::null => println!("Null:   {}", inner_pair.as_str()),
                _ => unreachable!(),
            };
        }
    }
}
