#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StringKind {
    Literal,
    Interpolation,
}
