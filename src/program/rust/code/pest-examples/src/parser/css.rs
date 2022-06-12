use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/css.pest"]
pub struct CssParser;

#[test]
fn test_float() {
    use pest::Parser;
    let pairs = CssParser::parse(Rule::float, "1.1").unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs {
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());
        // println!("tokens:    {:#?}", pair.tokens());

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::float => println!("float:  {}", inner_pair.as_str()),
                _ => unreachable!(),
            };
        }
    }
}

#[test]
fn test_length() {
    use pest::Parser;
    let pairs = CssParser::parse(Rule::length, "1.1px").unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::px => {
                    for inner_pair in inner_pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::float => println!("px:  {}", inner_pair.as_str()),
                            _ => break,
                        };
                    }
                }
                Rule::mm => {
                    for inner_pair in inner_pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::float => println!("mm:  {}", inner_pair.as_str()),
                            _ => break,
                        };
                    }
                }
                Rule::cm => {
                    for inner_pair in inner_pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::float => println!("cm:  {}", inner_pair.as_str()),
                            _ => break,
                        };
                    }
                }
                _ => break,
            };
        }
    }
}

#[test]
fn test_percentage() {
    use pest::Parser;
    let pairs = CssParser::parse(Rule::percentage, "1.1%").unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::float => println!("float:  {}", inner_pair.as_str()),
                _ => break,
            };
        }
    }
}

#[test]
fn test_border() {
    use pest::Parser;
    let pairs = CssParser::parse(Rule::border, "1.1px").unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::px => {
                    for inner_pair in inner_pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::float => println!("px:  {}", inner_pair.as_str()),
                            _ => break,
                        };
                    }
                }
                Rule::mm => {
                    for inner_pair in inner_pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::float => println!("mm:  {}", inner_pair.as_str()),
                            _ => break,
                        };
                    }
                }
                Rule::cm => {
                    for inner_pair in inner_pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::float => println!("cm:  {}", inner_pair.as_str()),
                            _ => break,
                        };
                    }
                }
                _ => break,
            };
        }
    }
}
