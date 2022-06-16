use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "parser/css.pest"]
pub struct CssParser;

pub use Rule as CssRule;

#[derive(Debug, Error, PartialEq)]
pub enum CssError {
    #[error("parse failed: {0}")]
    ParseError(pest::error::Error<CssRule>),
}

pub type CssResult<T> = Result<T, CssError>;

#[test]
fn test_css() {}
