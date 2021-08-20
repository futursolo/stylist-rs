use std::borrow::Cow;

use yew::html::IntoPropValue;

use crate::ast::Sheet;

impl<'a> IntoPropValue<Sheet> for String {
    fn into_prop_value(self) -> Sheet {
        self.parse::<Sheet>().expect("Failed to parse style.")
    }
}

impl<'a> IntoPropValue<Sheet> for &'a str {
    fn into_prop_value(self) -> Sheet {
        self.parse::<Sheet>().expect("Failed to parse style.")
    }
}

impl<'a> IntoPropValue<Sheet> for Cow<'a, str> {
    fn into_prop_value(self) -> Sheet {
        self.parse::<Sheet>().expect("Failed to parse style.")
    }
}

impl<'a> IntoPropValue<Sheet> for &'a Sheet {
    fn into_prop_value(self) -> Sheet {
        self.clone()
    }
}
