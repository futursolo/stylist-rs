#![deny(clippy::all)]
#![deny(missing_debug_implementations)]
#![deny(unsafe_code)]
#![deny(non_snake_case)]
#![cfg_attr(documenting, feature(doc_cfg))]
#![cfg_attr(any(releasing, not(debug_assertions)), deny(dead_code, unused_imports))]

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
//!     fn print_style(&self) {
//!         println!("{}", self.style().get_class_name());
//!     }
//! }
//!
//! ```
//!
//! Everything that is not in a conditioned block will be applied to the Component
//! the class of this style is applied to.
//!
//! You may also use Current Selector (`&`) in CSS selectors to denote the container element:
//!
//! ```css
//! &:hover {
//!   background-color: #d0d0d9;
//! }
//! ```
//!
//! You can also use other CSS rules(such as: keyframes, supports and media):
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
//! ```css
//! @supports (backdrop-filter: blur(5px)) {
//!   backdrop-filter: blur(5px);
//! }
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
//!
//! ## Features Flags
//!
//! - `random`: Enabled by default, this flag uses `rand` crate to generate a random
//!   class name. Disabling this flag will opt for a class name that is counter-based.
//! - `yew_integration`: This flag enables yew integration, which implements [`Classes`](::yew::html::Classes) for
//!   [`Style`] and provides a [`Global`](yew::Global) component for applying global styles.

#[cfg(target_arch = "wasm32")]
#[path = "arch_wasm.rs"]
mod arch;

pub mod manager;
mod registry;

mod global_style;
mod style;
mod utils;
mod yield_style;

pub use global_style::GlobalStyle;
pub use style::Style;
pub use yield_style::YieldStyle;

#[cfg_attr(documenting, doc(cfg(feature = "yew_integration")))]
#[cfg(feature = "yew_integration")]
pub mod yew;

#[doc(inline)]
pub use stylist_core::ast;
#[doc(inline)]
pub use stylist_core::{Error, Result};
