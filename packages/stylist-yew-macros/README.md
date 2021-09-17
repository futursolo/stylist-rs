## Stylist Yew Macros

This crate exists mainly to work around with documentation, otherwise it should be understood to be part of ::stylist::yew.

To be more specific, `#[macro_export] macro_rules!` exports a macro at the crate root, which also shows up documentation, unless marked as `#[doc(hidden)]`, which on the other hand is infectuous and hides also reexports, which we'd want to do to use it as `::stylist::yew::use_stylist!`. They also can't be exported from proc-macro crates, i.e. `stylist-macros` yet.

Tracking issue:
- rustdoc: doc(hidden) also hides re-exports: https://github.com/rust-lang/rust/issues/59368
