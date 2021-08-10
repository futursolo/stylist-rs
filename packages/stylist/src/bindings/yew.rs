//! This module contains yew specific features.

use std::borrow::Cow;

use yew::html::Classes;
use yew::html::IntoPropValue;
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::arch::document;
use crate::Style;

impl From<Style> for Classes {
    fn from(style: Style) -> Self {
        let mut classes = Self::new();
        classes.push(style.get_class_name().to_string());
        classes
    }
}

impl IntoPropValue<Style> for String {
    fn into_prop_value(self) -> Style {
        self.parse().expect("Failed to parse style.")
    }
}

impl IntoPropValue<Style> for &str {
    fn into_prop_value(self) -> Style {
        self.parse().expect("Failed to parse style.")
    }
}

impl IntoPropValue<Style> for Cow<'_, str> {
    fn into_prop_value(self) -> Style {
        self.parse().expect("Failed to parse style.")
    }
}

// /// A struct that implements [`From`] for both Style and anything that implements [`AsRef<str>`] and panics when
// /// failed to parse the string into a stylesheet.
// #[derive(Debug, Clone)]
// pub struct StyleOrPanic {
//     inner: Style,
// }

// impl From<Style> for StyleOrPanic {
//     fn from(style: Style) -> Self {
//         Self { inner: style }
//     }
// }

// impl<T: AsRef<str>> From<T> for StyleOrPanic {
//     fn from(style: T) -> Self {
//         Self {
//             inner: style.as_ref().parse().expect("Failed to parse style."),
//         }
//     }
// }

// impl IntoPropValue<StyleOrPanic> for String {
//     fn into_prop_value(self) -> StyleOrPanic {
//         self.into()
//     }
// }

// impl IntoPropValue<StyleOrPanic> for &str {
//     fn into_prop_value(self) -> StyleOrPanic {
//         self.into()
//     }
// }

// impl IntoPropValue<StyleOrPanic> for Cow<'_, str> {
//     fn into_prop_value(self) -> StyleOrPanic {
//         self.into()
//     }
// }

// impl Deref for StyleOrPanic {
//     type Target = Style;
//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }

/// The properties for [`GlobalStyle`] Component, please see its documentation for usage.
#[derive(Properties, Clone, Debug)]
pub struct GlobalStyleProps {
    pub css: Style,
}

/// A Global Style that will be applied to `<html />` tag, inspired by [emotion](https://emotion.sh).
///
/// The `css` attribute accepts either a [`Style`] or a string variant and panics if the string
/// failed to be parsed into a stylesheet.
///
/// # Example:
///
/// ```
/// use yew::prelude::*;
/// use stylist::yew::GlobalStyle;
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
///                 <GlobalStyle css="color: red;" />
///                 <div>{"Hello World!"}</div>
///             </>
///         }
///     }
/// }
/// ```
#[derive(Debug)]
pub struct GlobalStyle {
    props: GlobalStyleProps,

    #[cfg(target_arch = "wasm32")]
    style_class: Option<String>,
}

impl Component for GlobalStyle {
    type Message = ();
    type Properties = GlobalStyleProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,

            #[cfg(target_arch = "wasm32")]
            style_class: None,
        }
    }

    #[cfg(target_arch = "wasm32")]
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

#[cfg(target_arch = "wasm32")]
impl GlobalStyle {
    fn update_global_style(&mut self) {
        let next_style = self.props.css.get_class_name();
        if Some(next_style) == self.style_class.as_deref() {
            return;
        }

        let html_element = document()
            .ok()
            .and_then(|m| m.document_element())
            .expect("Failed to get <html /> element.");
        let html_class_list = html_element.class_list();

        if let Some(ref m) = self.style_class {
            html_class_list
                .remove_1(m)
                .expect("Failed to remove existing class.");
        }

        html_class_list
            .add_1(&next_style)
            .expect("Failed to add class.");

        self.style_class = Some(next_style.to_string());
    }
}
