use yew::prelude::*;

use crate::yew::IntoStyle;
use crate::GlobalStyle;

/// The properties for [`Global`] Component, please see its documentation for usage.
#[derive(Properties, Clone, Debug)]
pub struct GlobalProps {
    pub css: IntoStyle,
}

/// A Global Style that will be applied to `<html />` tag, inspired by [emotion](https://emotion.sh).
///
/// The `css` attribute accepts anything that implements
/// [`IntoPropValue<IntoStyle>`](yew::html::IntoPropValue) and
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
#[derive(Debug)]
pub struct Global {
    props: GlobalProps,

    global_style: Option<GlobalStyle>,
}

impl Component for Global {
    type Message = ();
    type Properties = GlobalProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,

            global_style: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.update_global_style();
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;

        #[cfg(target_arch = "wasm32")]
        self.update_global_style();

        false
    }

    fn view(&self) -> Html {
        Html::default()
    }
}

impl Global {
    fn update_global_style(&mut self) {
        if let Some(ref m) = self.global_style {
            m.unregister();
        }

        self.global_style = Some(
            GlobalStyle::new(self.props.css.to_sheet().as_ref()).expect("Failed to parse style."),
        );
    }
}
