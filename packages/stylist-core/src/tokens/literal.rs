use arcstr::Substr;

use super::Location;
use crate::{__impl_partial_eq, __impl_token};

#[derive(Debug, Clone)]
pub struct Literal {
    inner: Substr,
    location: Location,
}

__impl_partial_eq!(Literal, inner);
__impl_token!(Literal);
