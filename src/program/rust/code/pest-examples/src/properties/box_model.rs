use super::{number::*, CssProp};
use crate::parser::*;

pub struct Width(LengthPercentage);

impl CssProp for Width {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::width, i).ok() {
            Width::parse(pairs.last().unwrap())
        } else {
            None
        }
    }
    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        match pair.as_rule() {
            CssRule::length_percentage => LengthPercentage::parse(pair).map(|v| Self(v)),
            _ => None,
        }
    }
    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.0.to_css(dest)
    }
}

pub struct Height(LengthPercentage);

impl CssProp for Height {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::height, i).ok() {
            Height::parse(pairs.last().unwrap())
        } else {
            None
        }
    }
    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        match pair.as_rule() {
            CssRule::length_percentage => LengthPercentage::parse(pair).map(|v| Self(v)),
            _ => None,
        }
    }
    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.0.to_css(dest)
    }
}

pub struct Padding(LengthPercentage);

impl CssProp for Padding {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::padding, i).ok() {
            Padding::parse(pairs.last().unwrap())
        } else {
            None
        }
    }
    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        match pair.as_rule() {
            CssRule::length_percentage => LengthPercentage::parse(pair).map(|v| Self(v)),
            _ => None,
        }
    }
    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.0.to_css(dest)
    }
}
