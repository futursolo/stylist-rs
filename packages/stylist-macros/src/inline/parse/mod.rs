use crate::output::{OutputAtRule, OutputAttribute, OutputFragment, OutputQualifiedRule};
use itertools::Itertools;
use syn::parse::Error as ParseError;

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
                    attributes,
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
    let left_ends_compound = matches!(l, Delimiter(_, false) | Token(Ident(_)) | Token(Literal(_)))
        || matches!(l, Token(Punct(ref p)) if p.as_char() == '*');
    let right_starts_compound = matches!(r, Token(Ident(_)) | Token(Literal(_)))
        || matches!(r, Token(Punct(ref p)) if "*#".contains(p.as_char()));
    let needs_spacing = left_ends_compound && right_starts_compound;
    needs_spacing.then(|| ' '.into())
}
