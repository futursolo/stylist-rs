# Changelog

## v0.10.1

### Other Changes:
- Added an impl of `IntoPropValue<Classes>` for `Style` and `StyleSource` when
  the `yew_integration` feature is active.

## v0.10.0

### Breaking Changes:
- Yew version is bumped to 0.19.

### Other Changes:
- Added an API to style Yew Function Component.
- `random` features is now provided with `fastrand`.
- Added Yew hooks for Media Query.
- Fixed a bug where URL might not be parsed properly.

## v0.9.2

### Other Changes:
- Fixed a misconfiguration causing documentation fails to build on `docs.rs`.

## v0.9.1

### Other Changes:
- Removed an unused import.

## v0.9

### Breaking Changes:
- `Style` and `GlobalStyle` no longer implements `FromStr`.
- `Style` and `GlobalStyle` now takes any type that implements
  `Into<StyleSource>` as a source for a stylesheet.
- `style_str` method in `YieldStyle` renamed to `style_from`
  and returns a `StyleSource<'static>`.
- Accepted at-rules are limited to `@keyframe`, `@supports` and
  `@media`.

### Other Changes:
- Added a Procedural Macro API that parses the Stylesheet at the compile
  time.
- Parser will now check stylesheets more strictly.
- Parsed results are now cached.
- Updated `nom` to `v7`.
- Runtime parser is now optional (disabling `parser` will make the bundle
  ~70K smaller).
- Fixed comment handling.
- Panic-based behaviour now displays the error with `{}`(`std::fmt::Display`)
  in browser developer console.
- `@supports` and `@media` can now appear in a `Block`.

## v0.8

### Breaking Changes:
- `Style::new()` and `Style::create()` now takes a new trait `IntoSheet` for
  Stylesheet which is implemented by default for both
`stylist::ast::Sheet` and everything that implements `AsRef<str>`.
- Feature `yew` has been renamed back to `yew_integration`.
- Selectors list now gets a class name added for each selector.
- `Style` is now `!Send` and `!Sync`.
- Stylist now treats pseudo class selectors (e.g.:`:hover`) like emotion
  and styled-components.

### Other Changes:
- Added a `GlobalStyle` struct to register global styles.
- Added a `<Global />` Component for global styling for yew applications.
- Supported `@supports` CSS at-rule.
- Added an alternative counter-based class name on the style when
  feature `random` is disabled.
- Added a `StyleManager` type to manage the behaviour of styles.
- Moved AST and Parser logic to a new crate `stylist_core`.
- AST has been exposed under `stylist::ast`.
- Improved performance for looking up cached styles.
- Improved Examples.
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
