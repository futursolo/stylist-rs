use yew::prelude::*;

use crate::manager::StyleManager;
use crate::{GlobalStyle, StyleSource};
use stylist_core::ResultDisplay;

/// The properties for [`Global`] Component, please see its documentation for usage.
#[derive(Properties, Clone, Debug, PartialEq)]
pub struct GlobalProps {
    pub css: StyleSource<'static>,
}

/// A Global Style that will be applied to `<html />` tag, inspired by [emotion](https://emotion.sh).
///
/// The `css` attribute accepts anything that implements
/// [`IntoPropValue<StyleSource>`](yew::html::IntoPropValue) and
/// panics if the string failed to be parsed into a stylesheet.
///
/// # Example:
///
/// ```
/// use yew::prelude::*;
/// use stylist::yew::Global;
///
/// struct App;
///
/// impl Component for App {
///     type Message = ();
///     type Properties = ();
///
///     fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
///         Self
///     }
///
///     fn update(&mut self, _msg: Self::Message) -> ShouldRender {
///         false
///     }
///
///     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
///         false
///     }
///
///     fn view(&self) -> Html {
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
        css: StyleSource<'static>,
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
