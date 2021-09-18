use yew::prelude::*;

use stylist_core::ResultDisplay;

use crate::manager::StyleManager;
use crate::{Style, StyleSource};

/// A hook to create auto updating [`Style`]s.
///
/// # Example
///
/// ```
/// use yew::prelude::*;
/// use stylist::yew::use_style;
///
/// #[function_component(Comp)]
/// fn comp() -> Html {
///     // Returns a Style instance.
///     let style = use_style("color: red;");
///     html!{<div class={style}>{"Hello world!"}</div>}
/// }
/// ```
#[cfg_attr(documenting, doc(cfg(feature = "yew_use_style")))]
#[cfg(feature = "yew_use_style")]
pub fn use_style<'a, Css: Into<StyleSource<'a>>>(css: Css) -> Style {
    let mgr = use_context::<StyleManager>().unwrap_or_default();
    let css = css.into();

    let created_style: Style =
        Style::new_with_manager(css.clone(), mgr).expect_display("failed to create style");

    let style = use_state(|| created_style.clone());

    if style.id() != created_style.id() {
        style.set(created_style);
    }

    // It does not make sense to unmount a scoped style.

    (*style).clone()
}
