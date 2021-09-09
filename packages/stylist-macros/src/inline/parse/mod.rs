use crate::output::OutputFragment;

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
