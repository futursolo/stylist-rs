use std::borrow::Borrow;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;

#[cfg(feature = "parser")]
use crate::tokens::TokenStream;

/// A smart reference that lives at least `'a`.
///
/// This type behaves like [`Cow`](std::borrow::Cow) but uses [`Arc`] to store the owned value so
/// it's cheap to clone.
#[derive(Clone)]
pub enum ArcRef<'a, T>
where
    T: ?Sized + ToOwned,
{
    Arc(Arc<<T as ToOwned>::Owned>),
    Ref(&'a T),
}

impl<T> ArcRef<'_, T>
where
    T: ?Sized + ToOwned,
{
    /// Extracts the owned data.
    ///
    /// Clones the data if it is not already [`Arc`]ed or Arc fails to unwrap.
    pub fn into_owned(self) -> <T as ToOwned>::Owned {
        match self {
            Self::Arc(m) => Arc::try_unwrap(m).unwrap_or_else(|e| (*e).borrow().to_owned()),
            Self::Ref(m) => m.to_owned(),
        }
    }

    // /// Turns [`ArcRef<'_, T>`] into an [`Arc<T>`].
    // ///
    // /// Clones the data if it is not already [`Arc`]ed.
    // pub fn to_arc(&self) -> Arc<<T as ToOwned>::Owned> {
    //     match self {
    //         Self::Arc(ref m) => m.clone(),
    //         Self::Ref(m) => Arc::new((*m).to_owned()),
    //     }
    // }

    /// Extends the lifetime of an `ArcRef<'_, T>` into an `ArcRef<'static, T>`.
    ///
    /// Clones the data if it is not already [`Arc`]ed.
    pub fn into_static(self) -> ArcRef<'static, T> {
        match self {
            Self::Arc(m) => ArcRef::Arc(m),
            Self::Ref(m) => ArcRef::Arc(Arc::new(m.to_owned())),
        }
    }
}

impl<'a> From<String> for ArcRef<'a, str> {
    fn from(m: String) -> Self {
        Self::Arc(Arc::new(m))
    }
}

impl<'a> From<&'a str> for ArcRef<'a, str> {
    fn from(m: &'a str) -> Self {
        Self::Ref(m)
    }
}

impl<'a> From<&'a String> for ArcRef<'a, str> {
    fn from(m: &'a String) -> Self {
        Self::Ref(m)
    }
}

impl<T> Deref for ArcRef<'_, T>
where
    T: ?Sized + ToOwned,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ref(r) => r,
            Self::Arc(b) => (**b).borrow(),
        }
    }
}

impl<T> AsRef<T> for ArcRef<'_, T>
where
    T: ?Sized + ToOwned,
{
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T> PartialEq for ArcRef<'_, T>
where
    T: ?Sized + PartialEq + ToOwned,
{
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&**self, &**other)
    }
}

impl<T> Eq for ArcRef<'_, T> where T: ?Sized + Eq + ToOwned {}

impl<T> Hash for ArcRef<'_, T>
where
    T: ?Sized + Hash + ToOwned,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&**self, state)
    }
}

impl<T> fmt::Debug for ArcRef<'_, T>
where
    T: ?Sized + ToOwned + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Arc(ref b) => fmt::Debug::fmt((**b).borrow(), f),
            Self::Ref(ref o) => fmt::Debug::fmt(o, f),
        }
    }
}

#[cfg(feature = "parser")]
impl<'a> From<TokenStream> for ArcRef<'a, TokenStream> {
    fn from(m: TokenStream) -> Self {
        Self::Arc(Arc::new(m))
    }
}
