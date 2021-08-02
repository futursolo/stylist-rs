# Changelog

##v0.6

### Breaking Changes:
- `style.get_class_name()` no longer consumes the style and returns a `&str`
  instead of an owned string.
- Seed Integration is Removed.

- Added `Style::new` which does not require a component name.
- Aesthetically pleasing Class Name.
- Replaced `lazy_static` with `once_cell`.
- Updated nom to `v6.2.1`.
- Update Yew to `v0.18`.
