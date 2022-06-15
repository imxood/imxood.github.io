use super::CssProp;
use crate::parser::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Selector {
    /// # 开头, 唯一Id匹配的元素
    Id(String),
    /// . 开头,
    Class(String),
    /// 纯字符串
    Tag(String),
    /// * 表示所有子元素
    All,
    /// 组合多种选择器
    /// 如: "#user .friend" 表示 所有id为user的元素下, 所有的class为friend的元素
    Combinator(Vec<Selector>),
}

impl CssProp for Selector {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::selector, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        let mut v = pair
            .into_inner()
            .filter_map(|v| match v.as_rule() {
                CssRule::id => Some(Self::Id(v.as_str().to_string())),
                CssRule::class_name => Some(Self::Class(v.as_str().to_string())),
                CssRule::tagname => Some(Self::Tag(v.as_str().to_string())),
                CssRule::star => Some(Self::All),
                _ => None,
            })
            .collect::<Vec<_>>();
        let len = v.len();
        if len == 1 {
            return v.pop();
        }
        if len > 1 {
            return Some(Self::Combinator(v));
        }
        None
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        match self {
            Selector::Id(id) => dest.write_fmt(format_args!("#{}", id)),
            Selector::Class(class) => dest.write_fmt(format_args!(".{}", class)),
            Selector::Tag(tag) => dest.write_str(tag),
            Selector::All => dest.write_char('*'),
            Selector::Combinator(v) => {
                let len = v.len();
                for (i, selector) in v.iter().enumerate() {
                    selector.to_css(dest)?;
                    if i + 1 != len {
                        dest.write_char(' ')?;
                    }
                }
                Ok(())
            }
        }
    }
}

#[test]
fn test_selector() {
    let selector = Selector::parse_str("#id1");
    assert_eq!(selector, Some(Selector::Id("id1".to_string()),));

    let selector = Selector::parse_str("#id1 .user div");
    assert_eq!(
        selector,
        Some(Selector::Combinator(vec![
            Selector::Id("id1".to_string()),
            Selector::Class("user".to_string()),
            Selector::Tag("div".to_string())
        ]))
    );

    assert_eq!(selector.unwrap().to_css_string().as_str(), "#id1 .user div");
}
