use crate::parser::*;
use crate::properties::*;

use super::property::Properties;
use super::selector::Selector;

use derive_more::Constructor;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Entities(pub Vec<Entity>);

impl CssProp for Entities {
    fn parse_str(i: &str) -> Option<Self> {
        match CssParser::parse(CssRule::entities, i) {
            Ok(pairs) => Self::parse(pairs.last().unwrap()),
            Err(e) => panic!("error: {:?}", e),
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        let mut entities = Entities::default();
        for pair in pair.into_inner() {
            Entity::parse(pair).map(|v| entities.0.push(v));
        }
        Some(entities)
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        for entity in self.0.iter() {
            entity.to_css(dest)?;
            dest.write_str("\r\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone, Constructor)]
pub struct Entity {
    pub selectors: Vec<Selector>,
    pub properties: Properties,
}

impl CssProp for Entity {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::entity, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        let mut entity = Self::default();
        let mut changed = false;
        for pair in pair.into_inner() {
            match pair.as_rule() {
                CssRule::selectors => {
                    for pair in pair.into_inner() {
                        Selector::parse(pair).map(|v| entity.selectors.push(v));
                    }
                    changed = true;
                }
                CssRule::properties => {
                    Properties::parse(pair).map(|v| {
                        entity.properties = v;
                    });
                    changed = true;
                }
                _ => {}
            }
        }
        if changed {
            Some(entity)
        } else {
            None
        }
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        let selector_len = self.selectors.len();
        for (i, selector) in self.selectors.iter().enumerate() {
            selector.to_css(dest)?;
            if i + 1 == selector_len {
                dest.write_char(' ')?;
            } else {
                dest.write_str(", ")?;
            }
        }
        self.properties.to_css(dest)?;
        Ok(())
    }
}

#[test]
fn test_entity() {
    use crate::properties::box_model::*;
    use crate::properties::property::*;

    let entity = Entity::parse_str("#id1 .user div, #id2 .user div { width: 100px; height: 200em}");
    assert_eq!(
        entity,
        Some(Entity::new(
            vec![
                Selector::Combinator(vec![
                    Selector::Id("id1".to_string()),
                    Selector::Class("user".to_string()),
                    Selector::Tag("div".to_string())
                ]),
                Selector::Combinator(vec![
                    Selector::Id("id2".to_string()),
                    Selector::Class("user".to_string()),
                    Selector::Tag("div".to_string())
                ])
            ],
            Properties::new(vec![
                Property::Width(Width::new(LengthPercentage::Length(Length::Px(Px::new(
                    100.0
                ))))),
                Property::Height(Height::new(LengthPercentage::Length(Length::Em(Em::new(
                    200.0
                )))))
            ])
        ))
    );

    println!("entity: {:?}", entity.unwrap().to_css_string());
}

#[test]
fn test_entities() {
    use crate::properties::box_model::*;
    use crate::properties::property::*;

    let entity =
        Entities::parse_str("#id0 .user div, #id1 .user div { width: 100px; height: 200em} #id2 .user div, #id3 .user div { width: 100px; height: 200em}");
    assert_eq!(
        entity,
        Some(Entities(vec![
            Entity::new(
                vec![
                    Selector::Combinator(vec![
                        Selector::Id("id0".to_string()),
                        Selector::Class("user".to_string()),
                        Selector::Tag("div".to_string())
                    ]),
                    Selector::Combinator(vec![
                        Selector::Id("id1".to_string()),
                        Selector::Class("user".to_string()),
                        Selector::Tag("div".to_string())
                    ])
                ],
                Properties::new(vec![
                    Property::Width(Width::new(LengthPercentage::Length(Length::Px(Px::new(
                        100.0
                    ))))),
                    Property::Height(Height::new(LengthPercentage::Length(Length::Em(Em::new(
                        200.0
                    )))))
                ])
            ),
            Entity::new(
                vec![
                    Selector::Combinator(vec![
                        Selector::Id("id2".to_string()),
                        Selector::Class("user".to_string()),
                        Selector::Tag("div".to_string())
                    ]),
                    Selector::Combinator(vec![
                        Selector::Id("id3".to_string()),
                        Selector::Class("user".to_string()),
                        Selector::Tag("div".to_string())
                    ])
                ],
                Properties::new(vec![
                    Property::Width(Width::new(LengthPercentage::Length(Length::Px(Px::new(
                        100.0
                    ))))),
                    Property::Height(Height::new(LengthPercentage::Length(Length::Em(Em::new(
                        200.0
                    )))))
                ])
            )
        ]))
    );

    println!("entity: {:?}", entity.unwrap().to_css_string());
}
