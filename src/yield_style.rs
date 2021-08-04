use std::borrow::Cow;

use crate::{Result, Style};

pub trait YieldStyle {
    fn prefix(&self) -> Cow<'static, str> {
        "stylist".into()
    }

    fn style_str(&self) -> Cow<'static, str>;

    fn try_style(&self) -> Result<Style> {
        Style::create(self.prefix(), self.style_str())
    }

    fn style(&self) -> Style {
        self.try_style().expect("Failed to create style.")
    }

    fn try_style_class(&self) -> Result<String> {
        Ok(self.try_style()?.get_class_name().to_string())
    }

    fn style_class(&self) -> String {
        self.try_style_class().expect("Failed to create style.")
    }
}
