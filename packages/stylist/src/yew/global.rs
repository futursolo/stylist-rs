use yew::prelude::*;

use crate::manager::StyleManager;
use crate::{GlobalStyle, StyleSource};
use stylist_core::ResultDisplay;

/// The properties for the [`Global`] Component, please see its documentation for usage.
#[derive(Properties, Clone, Debug, PartialEq)]
pub struct GlobalProps {
    pub css: StyleSource,
}

/// A Global Style that will be applied to `<html />` tag, inspired by [emotion](https://emotion.sh).
///
/// The `css` attribute accepts a value of any type that implements
/// [`IntoPropValue<StyleSource>`](yew::html::IntoPropValue). If you use the `parser`
/// feature, this panics when supplying a string that fails to parse.
///
/// The style will be applied via the [`:root`](https://developer.mozilla.org/en-US/docs/Web/CSS/:root)
/// pseudo-class, which has higher specificity than "simply" using the `html` selector.
///
/// # Example:
///
/// ```
/// use stylist::css;
/// use stylist::yew::Global;
/// use yew::prelude::*;
///
/// #[function_component]
/// fn App() -> Html {
///     html! {
///         <>
///             <Global css={css!("color: red;")} />
///             <div>{"Hello World!"}</div>
///         </>
///     }
/// }
/// ```
#[function_component(Global)]
pub fn global(props: &GlobalProps) -> Html {
    let mgr = use_context::<StyleManager>().unwrap_or_default();

    #[derive(Debug, PartialEq)]
    struct GlobalDependents {
        manager: StyleManager,
        css: StyleSource,
    }

    use_effect_with_deps(
        |deps| {
            let global_style =
                GlobalStyle::new_with_manager(deps.css.clone(), deps.manager.clone())
                    .expect_display("Failed to create style.");

            move || global_style.unregister()
        },
        GlobalDependents {
            manager: mgr,
            css: props.css.clone(),
        },
    );

    Html::default()
}
