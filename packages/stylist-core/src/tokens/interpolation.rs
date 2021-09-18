use arcstr::Substr;

use super::Location;
use crate::{__impl_partial_eq, __impl_token};

#[derive(Debug, Clone)]
pub struct Interpolation {
    inner: Substr,
    location: Location,
}

__impl_partial_eq!(Interpolation, inner);
__impl_token!(Interpolation);
