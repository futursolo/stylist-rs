#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(non_snake_case)]
#![cfg_attr(documenting, feature(doc_cfg))]

//! Stylist is a CSS-in-Rust styling solution for WebAssembly Applications.
//!
//! ## Usage
//!
//! There're two APIs that you can use to style your components.
//!
//! ### Style API
//!
//! To create a stylesheet, you can use [`Style::new`]:
//!
//! ```rust
//! use stylist::Style;
//!
//! let style = Style::new(
//!     r#"
//!     background-color: red;
//!
//!     .nested {
//!         background-color: blue;
//!         width: 100px
//!     }"#,
//! ).expect("Failed to create style");
//! ```
//!
//! If you want to use a custom prefix for your class name,
//! you can use [`Style::create`].
//!
//! ```rust
//! use stylist::Style;
//!
//! let style = Style::create(
//!     "MyComponent",
//!     r#"
//!     background-color: red;
//!
//!     .nested {
//!         background-color: blue;
//!         width: 100px
//!     }"#,
//! ).expect("Failed to create style");
//! ```
//!
//! Everything that is not in a conditioned block will be applied to the Component
//! the class of this style is applied to.
//!
//! The Style you put in will get parsed and converted to actual CSS and automatically appended
//! to the head of your HTML document.
//!
//! You may also use the `&` identifier in order to use CSS selectors or pseudo
//! classes on the styled element:
//!
//! ```css
//! &:hover {
//!   background-color: #d0d0d9;
//! }
//! ```
//!
//! You can also use other CSS rules, e.g. keyframes:
//!
//! ```css
//! @keyframes mymove {
//!   from {
//!     top: 0px;
//!   }
//!   to {
//!     top: 200px;
//!   }
//! }
//! ```
//!
//! ```css
//! @media only screen and (max-width: 600px) {
//!   background-color: #303040;
//!
//!   .nested {
//!     background-color: lightblue;
//!   }
//!
//!   &:hover {
//!     background-color: #606072;
//!   }
//! }
//! ```
//!
//! ### YieldStyle API
//!
//! Alternatively, any struct that implements [`YieldStyle`] trait can call
//! `self.style()` to get a [`Style`] instance.
//!
//! ```rust
//! use std::borrow::Cow;
//! use stylist::YieldStyle;
//!
//! pub struct Component;
//!
//! impl YieldStyle for Component {
//!     fn style_str(&self) -> Cow<'static, str> {
//!         "color: red;".into()
//!     }
//! }
//!
//! impl Component {
//!     fn print_style(&self) -> Self {
//!         println!("{}", self.style().get_class_name());
//!
//!         unimplemented!();
//!     }
//! }
//!
//! ```
//!
//! ## Yew Integration
//!
//! To enable yew integration. Enable feature `yew` in `Cargo.toml`.
//!
//! Then create a style and use it with yew like this:
//!
//! ```rust
//! use std::borrow::Cow;
//!
//! use yew::prelude::*;
//! use stylist::YieldStyle;
//!
//! struct MyStyledComponent {}
//!
//! impl Component for MyStyledComponent {
//!     type Message = ();
//!     type Properties = ();
//!
//!     fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
//!         Self {}
//!     }
//!
//!     fn change(&mut self, _: Self::Properties) -> ShouldRender {
//!         false
//!     }
//!
//!     fn update(&mut self, _: Self::Message) -> ShouldRender {
//!         false
//!     }
//!
//!     fn view(&self) -> Html {
//!         html! {<div class=self.style()>{"Hello World!"}</div>}
//!     }
//! }
//!
//! impl YieldStyle for MyStyledComponent {
//!     fn style_str(&self) -> Cow<'static, str> {
//!         "color: red;".into()
//!     }
//! }
//! ```
//!
//! ### Theming
//!
//! There're theming examples using
//! [Yewdux](https://github.com/futursolo/stylist-rs/tree/master/examples/yew-theme-yewdux)
//! and [yewtil::store](https://github.com/futursolo/stylist-rs/tree/master/examples/yew-theme-agent).

mod ast;
#[doc(hidden)]
pub mod bindings;
mod error;
mod parser;
mod registry;
#[doc(hidden)]
pub mod style;
mod utils;
#[doc(hidden)]
pub mod yield_style;

pub use error::{Error, Result};
pub use style::Style;
pub use yield_style::YieldStyle;
