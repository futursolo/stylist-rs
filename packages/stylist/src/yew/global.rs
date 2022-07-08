use yew::prelude::*;

use crate::manager::StyleManager;
use crate::{GlobalStyle, StyleSource};
use stylist_core::ResultDisplay;

/// The properties for [`Global`] Component, please see its documentation for usage.
#[derive(Properties, Clone, Debug, PartialEq)]
pub struct GlobalProps {
    pub css: StyleSource,
}

/// A Global Style that will be applied to `<html />` tag, inspired by [emotion](https://emotion.sh).
///
/// The `css` attribute accepts type that implements
/// [`IntoPropValue<StyleSource>`](yew::html::IntoPropValue) and
/// panics if the string failed to be parsed into a stylesheet.
///
/// # Example:
///
/// ```
/// use stylist::yew::Global;
/// use yew::prelude::*;
///
/// struct App;
///
/// impl Component for App {
///     type Message = ();
///     type Properties = ();
///
///     fn create(_ctx: &Context<Self>) -> Self {
///         Self
///     }
///
///     fn view(&self, _ctx: &Context<Self>) -> Html {
///         html! {
///             <>
///                 <Global css="color: red;" />
///                 <div>{"Hello World!"}</div>
///             </>
///         }
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
