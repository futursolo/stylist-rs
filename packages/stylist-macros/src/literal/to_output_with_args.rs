use std::collections::{HashMap, HashSet};

use proc_macro_error::abort_call_site;

use stylist_core::ast::*;

use crate::output::{
    OutputAtRule, OutputAttribute, OutputBlockContent, OutputFragment, OutputQualifiedRule,
    OutputQualifier, OutputRuleBlock, OutputRuleBlockContent, OutputRuleContent,
    OutputScopeContent, OutputSelector, OutputSheet,
};

use super::{argument::Argument, fstring};

pub(crate) trait ToOutputWithArgs {
    type Output;

    fn to_output_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> Self::Output;
}

impl ToOutputWithArgs for Selector {
    type Output = OutputSelector;

    fn to_output_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> Self::Output {
        let mut selectors = Vec::new();

        for frag in self.fragments.iter() {
            selectors.extend(frag.to_output_with_args(args, args_used));
        }
        OutputSelector { selectors }
    }
}

impl ToOutputWithArgs for RuleBlock {
    type Output = OutputRuleBlock;

    fn to_output_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> Self::Output {
        let mut condition = Vec::new();

        for i in self.condition.iter() {
            condition.extend(i.to_output_with_args(args, args_used));
        }

        let mut contents = Vec::new();

        for i in self.content.iter() {
            contents.push(i.to_output_with_args(args, args_used));
        }

        OutputRuleBlock {
            condition,
            content: contents,
            // errors: Vec::new(),
        }
    }
}

impl ToOutputWithArgs for RuleBlockContent {
    type Output = OutputRuleBlockContent;

    fn to_output_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> Self::Output {
        match self {
            Self::RuleBlock(ref m) => {
                let block = m.to_output_with_args(args, args_used);
                OutputRuleBlockContent::RuleBlock(Box::new(block))
            }
            Self::StyleAttr(ref m) => {
                let rule = m.to_output_with_args(args, args_used);
                OutputRuleBlockContent::StyleAttr(rule)
            }
        }
    }
}

impl ToOutputWithArgs for BlockContent {
    type Output = OutputBlockContent;

    fn to_output_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> Self::Output {
        match self {
            Self::RuleBlock(ref m) => {
                let block = m.to_output_with_args(args, args_used);
                OutputBlockContent::RuleBlock(block)
            }
            Self::StyleAttr(ref m) => {
                let rule = m.to_output_with_args(args, args_used);
                OutputBlockContent::StyleAttr(rule)
            }
        }
    }
}

impl ToOutputWithArgs for StyleAttribute {
    type Output = OutputAttribute;

    fn to_output_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> Self::Output {
        let key = OutputFragment::Str(self.key.as_ref().to_string());

        let mut values = Vec::new();

        for i in self.value.iter() {
            values.extend(i.to_output_with_args(args, args_used));
        }

        OutputAttribute {
            key,
            values,
            errors: Vec::new(),
        }
    }
}

impl ToOutputWithArgs for Block {
    type Output = OutputQualifiedRule;

    fn to_output_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> Self::Output {
        let mut selector_list = Vec::new();

        for i in self.condition.iter() {
            selector_list.push(i.to_output_with_args(args, args_used));
        }

        let mut content = Vec::new();

        for i in self.content.iter() {
            content.push(i.to_output_with_args(args, args_used));
        }

        OutputQualifiedRule {
            qualifier: OutputQualifier {
                selector_list,
                errors: Vec::new(),
            },
            content,
        }
    }
}

impl ToOutputWithArgs for RuleContent {
    type Output = OutputRuleContent;

    fn to_output_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> Self::Output {
        match self {
            Self::Block(ref m) => {
                let block = m.to_output_with_args(args, args_used);
                OutputRuleContent::Block(block)
            }
            Self::Rule(ref m) => {
                let rule = m.to_output_with_args(args, args_used);
                OutputRuleContent::AtRule(rule)
            }
            Self::String(ref m) => OutputRuleContent::String(m.as_ref().to_string()),
        }
    }
}

impl ToOutputWithArgs for StringFragment {
    type Output = Vec<OutputFragment>;

    fn to_output_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> Self::Output {
        let fragments = match fstring::Parser::parse(&self.inner) {
            Ok(m) => m,
            Err(e) => abort_call_site!("{}", e),
        };

        let mut fragments_out = Vec::new();

        for frag in fragments.iter() {
            match frag {
                fstring::Fragment::Literal(ref m) => {
                    fragments_out.push(OutputFragment::Str(m.clone()));
                }

                fstring::Fragment::Interpolation(ref m) => {
                    let arg = match args.get(m) {
                        Some(m) => m,
                        None => abort_call_site!("missing argument: {}", self.inner),
                    };

                    args_used.insert(arg.name.clone());
                    fragments_out.push(arg.into());
                }
            }
        }

        fragments_out
    }
}

impl ToOutputWithArgs for Rule {
    type Output = OutputAtRule;

    fn to_output_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> Self::Output {
        let mut prelude = Vec::new();

        for i in self.condition.iter() {
            prelude.extend(i.to_output_with_args(args, args_used));
        }

        let mut contents = Vec::new();

        for i in self.content.iter() {
            contents.push(i.to_output_with_args(args, args_used));
        }

        OutputAtRule {
            prelude,
            contents,
            errors: Vec::new(),
        }
    }
}

impl ToOutputWithArgs for ScopeContent {
    type Output = OutputScopeContent;

    fn to_output_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> Self::Output {
        match self {
            Self::Block(ref m) => {
                let block = m.to_output_with_args(args, args_used);
                OutputScopeContent::Block(block)
            }
            Self::Rule(ref m) => {
                let rule = m.to_output_with_args(args, args_used);
                OutputScopeContent::AtRule(rule)
            }
        }
    }
}

impl ToOutputWithArgs for Sheet {
    type Output = OutputSheet;

    fn to_output_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> Self::Output {
        let mut contents = Vec::new();

        for i in self.iter() {
            contents.push(i.to_output_with_args(args, args_used));
        }
        OutputSheet { contents }
    }
}
