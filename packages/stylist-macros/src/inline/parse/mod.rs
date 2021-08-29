use super::output::{
    OutputAtRule, OutputQualifiedRule, OutputQualifier, OutputRuleContent, OutputScopeContent,
    Reify,
};
use proc_macro2::TokenStream;
use std::iter::Peekable;
use syn::parse::Result as ParseResult;

mod root;
pub use root::CssRootNode;
mod scope;
pub use scope::CssScope;
mod scope_content;
pub use scope_content::CssScopeContent;
mod block;
pub use block::CssQualifiedRule;
mod qualifier;
pub use qualifier::CssBlockQualifier;
mod rule;
pub use rule::CssAtRule;
mod attribute;
pub use attribute::{CssAttribute, CssAttributeName, CssAttributeValue};

/// We want to normalize the input a bit. For that, we want to pretend that e.g.
/// the sample input
///
/// ```css
/// outer-attribute: some;
/// foo-bar: zet;
/// .nested {
///     @media print {
///         only-in-print: foo;
///     }
///     and-always: red;
/// }
/// ```
///
/// gets processed as if written in the (more verbose) shallowly nested style:
///
/// ```css
/// {
///     outer-attribute: some;
///     foo-bar: zet;
/// }
/// @media print {
///     .nested {
///         only-in-print: foo;
///     }
/// }
/// .nested {
///     and-always: red;
/// }
///
/// ```
///
/// Errors in nested items are reported as spanned TokenStreams.
///
fn fold_normalized_scope_hierarchy<'it>(
    it: impl 'it + IntoIterator<Item = CssScopeContent>,
) -> impl 'it + Iterator<Item = ParseResult<OutputSheetContent>> {
    fold_tokens_impl(Default::default(), it)
}

enum OutputSheetContent {
    AtRule(OutputAtRule),
    QualifiedRule(OutputQualifiedRule),
}

fn fold_tokens_impl<'it>(
    context: CssBlockQualifier,
    it: impl 'it + IntoIterator<Item = CssScopeContent>,
) -> impl 'it + Iterator<Item = ParseResult<OutputSheetContent>> {
    // Step one: collect attributes into blocks, flatten and lift nested blocks
    struct Wrapper<I: Iterator> {
        it: Peekable<I>,
        qualifier: OutputQualifier,
    }

    enum WrappedCase {
        AttributeCollection(OutputQualifiedRule),
        AtRule(CssAtRule),
        Qualified(CssQualifiedRule),
    }

    impl<I: Iterator<Item = CssScopeContent>> Iterator for Wrapper<I> {
        type Item = WrappedCase;
        fn next(&mut self) -> Option<Self::Item> {
            match self.it.peek() {
                None => return None,
                Some(CssScopeContent::Attribute(_)) => {
                    let mut attributes = Vec::new();
                    while let Some(CssScopeContent::Attribute(attr)) = self
                        .it
                        .next_if(|i| matches!(i, CssScopeContent::Attribute(_)))
                    {
                        attributes.push(attr.into_output().into_token_stream());
                    }
                    let replacement = OutputQualifiedRule {
                        qualifier: self.qualifier.clone().into_token_stream(),
                        attributes,
                    };
                    return Some(WrappedCase::AttributeCollection(replacement));
                }
                _ => {}
            }
            match self.it.next() {
                Some(CssScopeContent::AtRule(r)) => Some(WrappedCase::AtRule(r)),
                Some(CssScopeContent::Nested(b)) => Some(WrappedCase::Qualified(b)),
                _ => unreachable!("Should have returned after peeking"),
            }
        }
    }

    Wrapper {
        it: it.into_iter().peekable(),
        qualifier: context.clone().into_output(),
    }
    .flat_map(move |w| match w {
        WrappedCase::AttributeCollection(attrs) => {
            let result = Ok(OutputSheetContent::QualifiedRule(attrs));
            Box::new(std::iter::once(result))
        }
        WrappedCase::AtRule(r) => {
            let result = Ok(r.fold_in_context(context.clone()));
            Box::new(std::iter::once(result))
        }
        WrappedCase::Qualified(b) => b.fold_in_context(context.clone()),
    })
}

fn reify_scope_contents<
    O: From<OutputSheetContent> + Reify,
    It: Iterator<Item = ParseResult<OutputSheetContent>>,
>(
    scope: It,
) -> Vec<TokenStream> {
    scope.map(|i| i.map(O::from).into_token_stream()).collect()
}

impl From<OutputSheetContent> for OutputRuleContent {
    fn from(c: OutputSheetContent) -> Self {
        match c {
            OutputSheetContent::QualifiedRule(block) => Self::Block(block),
            OutputSheetContent::AtRule(rule) => Self::AtRule(rule),
        }
    }
}

impl From<OutputSheetContent> for OutputScopeContent {
    fn from(c: OutputSheetContent) -> Self {
        match c {
            OutputSheetContent::QualifiedRule(block) => Self::Block(block),
            OutputSheetContent::AtRule(rule) => Self::AtRule(rule),
        }
    }
}
