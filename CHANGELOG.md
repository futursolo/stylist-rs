# Changelog

## v0.8
### Breaking Changes:
- `Style::new()` and `Style::create()` now takes `AsRef<str>` for
  Stylesheet.
- Feature `yew` has been renamed back to `yew_integration`

### Other Changes:
- Added a `<GlobalStyle />` Component for global styling.
- Added an alternative counter-based class name on the style when
  feature `rand` is disabled.
- Moved AST and Parser logic to a new crate `stylist_core`.
- AST has been exposed under `stylist::ast`.
- Documentation now shows feature flags.

## v0.7

### Breaking Changes:
- `Style::new()` now takes an `Into<Cow<'static, str>>` instead of
  `Into<String>` and returns `stylist::Error` instead of `String` when
  encountering an error.
- `Style::create()` now takes `Into<Cow<'static, str>>` for class prefix
  and css string and returns `stylist::Error`instead of `String` when
  encountering an error.
- `Style` no longer implements `ToString`.

### Other Changes:
- Added a new API `YieldStyle`.
- Added theming examples.
- Styles are now cached by default.
- Fixed a Bug where `.a-class-name` is after `@media` would cause parser
  to return an error.
- Added Docs.
- Removed Unnecessary Clones.
- Optimised for Performance.


## v0.6

### Breaking Changes:
- `style.get_class_name()` no longer consumes the style and returns a `&str`
  instead of an owned string.
- Seed Integration is Removed.

### Other Changes:
- Added `Style::new` which does not require a component name.
- Aesthetically pleasing Class Name.
- Replaced `lazy_static` with `once_cell`.
- Updated nom to `v6`.
- Updated Yew to `v0.18`.
- Removed Unnecessary Clones.
- Optimised for Performance.
