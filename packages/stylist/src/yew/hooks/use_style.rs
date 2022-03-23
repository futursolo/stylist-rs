use yew::prelude::*;

use stylist_core::ResultDisplay;

use crate::manager::StyleManager;
use crate::{Style, StyleSource};

/// A hook to create auto updating [`Style`]s.
///
/// # Example
///
/// ```
/// use stylist::yew::use_style;
/// use yew::prelude::*;
///
/// #[function_component(Comp)]
/// fn comp() -> Html {
///     // Returns a Style instance.
///     let style = use_style("color: red;");
///     html! {<div class={style}>{"Hello world!"}</div>}
/// }
/// ```
#[cfg(feature = "yew_use_style")]
#[hook]
pub fn use_style<Css>(css: Css) -> Style
where
    Css: TryInto<StyleSource>,
    crate::Error: From<Css::Error>,
{
    let mgr = use_context::<StyleManager>().unwrap_or_default();

    // It does not make sense to unmount a scoped style.
    Style::new_with_manager(css, mgr).expect_display("failed to create style")
}
