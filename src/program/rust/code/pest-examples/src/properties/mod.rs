use crate::parser::css::*;
use pest::iterators::Pair;

mod number;
mod box_model;
mod color;
mod display;

pub use number::*;

pub trait CssProp<T = Self> {
    fn parse_str(i: &str) -> Option<T>;

    fn parse(pair: Pair<Rule>) -> Option<T>;

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result;

    #[inline]
    fn to_css_string(&self) -> String {
        let mut s = String::new();
        self.to_css(&mut s).unwrap();
        s
    }
}
