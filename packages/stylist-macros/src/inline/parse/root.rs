use super::{CssAttribute, CssQualifiedRule, CssScopeContent};
use crate::output::{OutputScopeContent, OutputSheet};
use syn::parse::{Error as ParseError, Parse, ParseBuffer, Result as ParseResult};

#[derive(Debug)]
pub struct CssRootNode {
    contents: Vec<CssScopeContent>,
}

impl Parse for CssRootNode {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let contents = CssScopeContent::consume_list_of_rules(input)?;
        Ok(Self { contents })
    }
}

impl CssRootNode {
    pub fn into_output(self) -> Result<OutputSheet, Vec<ParseError>> {
        let mut errors = Vec::new();
        let mut contents = Vec::new();

        let mut attrs: Vec<CssAttribute> = Vec::new();

        let push_attrs_into_contents =
            |attrs: &mut Vec<CssAttribute>,
             contents: &mut Vec<OutputScopeContent>,
             errors: &mut Vec<ParseError>| {
                if attrs.is_empty() {
                    return;
                }

                match CssQualifiedRule::into_dangling_output(attrs) {
                    Ok(m) => contents.push(OutputScopeContent::Block(m)),
                    Err(e) => errors.extend(e),
                }
            };

        for scope in self.contents {
            match scope {
                CssScopeContent::Attribute(m) => {
                    attrs.push(m);
                }
                CssScopeContent::AtRule(m) => {
                    push_attrs_into_contents(&mut attrs, &mut contents, &mut errors);

                    match m.into_rule_output() {
                        Ok(m) => {
                            contents.push(OutputScopeContent::AtRule(m));
                        }
                        Err(e) => errors.extend(e),
                    };
                }

                CssScopeContent::Nested(m) => {
                    push_attrs_into_contents(&mut attrs, &mut contents, &mut errors);

                    match m.into_output() {
                        Ok(m) => {
                            contents.push(OutputScopeContent::Block(m));
                        }
                        Err(e) => errors.extend(e),
                    };
                }
            }
        }

        push_attrs_into_contents(&mut attrs, &mut contents, &mut errors);

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(OutputSheet { contents })
        }
    }
}
