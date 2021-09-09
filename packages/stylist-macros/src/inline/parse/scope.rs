use super::CssScopeContent;
use syn::{
    braced,
    parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult},
    token,
};

use crate::output::{
    OutputAttribute, OutputBlock, OutputBlockContent, OutputRuleBlockContent, OutputRuleContent,
};

#[derive(Debug)]
pub struct CssScope {
    brace: token::Brace,
    pub contents: Vec<CssScopeContent>,
}

impl Parse for CssScope {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let inner;
        let brace = braced!(inner in input);
        let contents = CssScopeContent::consume_list_of_rules(&inner)?;
        Ok(Self { brace, contents })
    }
}

impl CssScope {
    pub fn into_rule_output(self) -> Result<Vec<OutputRuleContent>, Vec<ParseError>> {
        let mut attrs = Vec::new();
        let mut errors = Vec::new();

        let mut contents = Vec::new();

        let collect_attrs_into_contents =
            |attrs: &mut Vec<OutputAttribute>, contents: &mut Vec<OutputRuleContent>| {
                if attrs.is_empty() {
                    return;
                }

                contents.push(OutputRuleContent::Block(OutputBlock {
                    condition: Vec::new(),
                    content: attrs
                        .drain(0..)
                        .map(OutputBlockContent::StyleAttr)
                        .collect(),
                }));
            };

        for scope in self.contents {
            match scope {
                CssScopeContent::Attribute(m) => match m.into_output() {
                    Ok(m) => attrs.push(m),
                    Err(e) => errors.extend(e),
                },
                CssScopeContent::AtRule(m) => {
                    collect_attrs_into_contents(&mut attrs, &mut contents);

                    match m.into_rule_output() {
                        Ok(m) => contents.push(OutputRuleContent::Rule(m)),
                        Err(e) => errors.extend(e),
                    }
                }
                CssScopeContent::Nested(m) => {
                    collect_attrs_into_contents(&mut attrs, &mut contents);

                    match m.into_output() {
                        Ok(m) => contents.push(OutputRuleContent::Block(m)),
                        Err(e) => errors.extend(e),
                    }
                }
            }
        }

        collect_attrs_into_contents(&mut attrs, &mut contents);

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(contents)
        }
    }

    pub fn into_rule_block_output(self) -> Result<Vec<OutputRuleBlockContent>, Vec<ParseError>> {
        let mut errors = Vec::new();
        let mut contents = Vec::new();

        for scope in self.contents {
            match scope {
                CssScopeContent::Attribute(m) => match m.into_output() {
                    Ok(m) => contents.push(OutputRuleBlockContent::StyleAttr(m)),
                    Err(e) => errors.extend(e),
                },
                CssScopeContent::AtRule(m) => match m.into_rule_block_output() {
                    Ok(m) => contents.push(OutputRuleBlockContent::RuleBlock(Box::new(m))),
                    Err(e) => errors.extend(e),
                },
                CssScopeContent::Nested(m) => {
                    errors.push(ParseError::new_spanned(
                        m.qualifier,
                        "Can not nest qualified blocks (yet)",
                    ));
                }
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(contents)
        }
    }

    pub fn into_block_output(self) -> Result<Vec<OutputBlockContent>, Vec<ParseError>> {
        let mut errors = Vec::new();
        let mut contents = Vec::new();

        for scope in self.contents {
            match scope {
                CssScopeContent::Attribute(m) => match m.into_output() {
                    Ok(m) => contents.push(OutputBlockContent::StyleAttr(m)),
                    Err(e) => errors.extend(e),
                },
                CssScopeContent::AtRule(m) => match m.into_rule_block_output() {
                    Ok(m) => contents.push(OutputBlockContent::RuleBlock(m)),
                    Err(e) => errors.extend(e),
                },
                CssScopeContent::Nested(m) => {
                    errors.push(ParseError::new_spanned(
                        m.qualifier,
                        "Can not nest qualified blocks (yet)",
                    ));
                }
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(contents)
        }
    }
}
