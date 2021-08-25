use std::borrow::Cow;

use yew::html::IntoPropValue;

use crate::IntoStyle;
use crate::Style;

impl IntoPropValue<Style> for String {
    fn into_prop_value(self) -> Style {
        Style::new(self).expect("Failed to parse style.")
    }
}

impl IntoPropValue<Style> for &str {
    fn into_prop_value(self) -> Style {
        Style::new(self).expect("Failed to parse style.")
    }
}

impl IntoPropValue<Style> for Cow<'_, str> {
    fn into_prop_value(self) -> Style {
        Style::new(self).expect("Failed to parse style.")
    }
}

impl IntoPropValue<IntoStyle> for String {
    fn into_prop_value(self) -> IntoStyle {
        IntoStyle::String(self.into())
    }
}

impl IntoPropValue<IntoStyle> for &'static str {
    fn into_prop_value(self) -> IntoStyle {
        IntoStyle::String(self.into())
    }
}

impl IntoPropValue<IntoStyle> for Cow<'static, str> {
    fn into_prop_value(self) -> IntoStyle {
        IntoStyle::String(self)
    }
}
