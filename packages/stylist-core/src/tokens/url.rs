use arcstr::Substr;

use super::Location;
use crate::{__impl_partial_eq, __impl_token};

#[derive(Debug, Clone)]
pub struct Url {
    inner: Substr,
    location: Location,
}

__impl_partial_eq!(Url, inner);
__impl_token!(Url);
