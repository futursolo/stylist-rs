use itertools::Itertools;
use syn::parse::Error as ParseError;

use crate::output::{
    OutputAtRule, OutputAttribute, OutputBlockContent, OutputFragment, OutputQualifiedRule,
};

mod attribute;
mod block;
mod qualifier;
mod root;
mod rule;
mod scope;
mod scope_content;

pub use attribute::{CssAttribute, CssAttributeName, CssAttributeValue};
pub use block::CssQualifiedRule;
pub use qualifier::CssBlockQualifier;
pub use root::CssRootNode;
pub use rule::CssAtRule;
pub use scope::CssScope;
pub use scope_content::CssScopeContent;

/// We want to normalize the input a bit. For that, we want to pretend that e.g.
/// the sample input
///
/// ```css
/// outer-attribute: some;
/// foo-bar: zet;
/// @media print {
///     .nested {
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
///     {
///         and-always: red;
///     }
/// }
/// ```
///
/// Errors in nested items are reported as spanned TokenStreams.
///
fn normalize_scope_hierarchy<'it>(
    it: impl 'it + IntoIterator<Item = CssScopeContent>,
) -> impl 'it + Iterator<Item = OutputSheetContent> {
    normalize_hierarchy_impl(Default::default(), it)
}

enum OutputSheetContent {
    AtRule(OutputAtRule),
    QualifiedRule(OutputQualifiedRule),
    Error(ParseError),
}

// Collect attributes into blocks, also flatten and lift nested blocks.
fn normalize_hierarchy_impl<'it>(
    context: CssBlockQualifier,
    it: impl 'it + IntoIterator<Item = CssScopeContent>,
) -> impl 'it + Iterator<Item = OutputSheetContent> {
    let qualifier = context.clone().into_output();

    // Helper enum appearing in intermediate step
    enum ScopeItem {
        Attributes(Vec<OutputAttribute>),
        AtRule(CssAtRule),
        Block(CssQualifiedRule),
    }
    it.into_iter()
        .map(|c| match c {
            CssScopeContent::Attribute(a) => ScopeItem::Attributes(vec![a.into_output()]),
            CssScopeContent::AtRule(r) => ScopeItem::AtRule(r),
            CssScopeContent::Nested(b) => ScopeItem::Block(b),
        })
        // collect runs of attributes together into a single item
        .coalesce(|l, r| match (l, r) {
            (ScopeItem::Attributes(mut ls), ScopeItem::Attributes(rs)) => {
                ls.extend(rs);
                Ok(ScopeItem::Attributes(ls))
            }
            (l, r) => Err((l, r)),
        })
        .flat_map(move |w| match w {
            ScopeItem::Attributes(attributes) => {
                let result = OutputSheetContent::QualifiedRule(OutputQualifiedRule {
                    qualifier: qualifier.clone(),
                    content: attributes
                        .into_iter()
                        .map(|m| OutputBlockContent::StyleAttr(m))
                        .collect(),
                });
                Box::new(std::iter::once(result))
            }
            ScopeItem::AtRule(r) => {
                let result = r.fold_in_context(context.clone());
                Box::new(std::iter::once(result))
            }
            ScopeItem::Block(b) => b.fold_in_context(context.clone()),
        })
}

pub fn fragment_spacing(l: &OutputFragment, r: &OutputFragment) -> Option<OutputFragment> {
    use super::component_value::PreservedToken::*;
    use OutputFragment::*;
    let needs_spacing = matches!(
        (l, r),
        (Delimiter(_, false), Token(Ident(_)))
            | (
                Token(Ident(_)) | Token(Literal(_)),
                Token(Ident(_)) | Token(Literal(_))
            )
    );
    needs_spacing.then(|| ' '.into())
}
