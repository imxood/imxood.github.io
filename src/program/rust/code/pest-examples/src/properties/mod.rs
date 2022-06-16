use crate::parser::*;

mod number;
mod box_model;
mod color;
mod display;
mod selector;
mod property;
pub mod entry;

pub use number::*;

pub trait CssProp<T = Self> {
    fn rule() -> CssRule;

    fn parse(pair: Pair<CssRule>) -> T;

    fn parse_str(i: &str) -> CssResult<T> {
        match CssParser::parse(Self::rule(), i) {
            Ok(pairs) => Ok(Self::parse(pairs.last().unwrap())),
            Err(e) => Err(CssError::ParseError(e)),
        }
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result;

    #[inline]
    fn to_css_string(&self) -> String {
        let mut s = String::new();
        self.to_css(&mut s).unwrap();
        s
    }
}
