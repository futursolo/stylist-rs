use std::hash::{Hash, Hasher};
use std::ops::Deref;
use Bow::*;

/// A clone-on-write smart pointer with boxed owned data.
///
/// This type behaves like [`Cow`](std::borrow::Cow) but uses [`Box`] to store the owned value.
#[derive(Debug)]
pub enum Bow<'a, T: 'a + ?Sized> {
    Borrowed(&'a T),
    Boxed(Box<T>),
}

impl<T: ?Sized> Deref for Bow<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Borrowed(r) => r,
            Boxed(ref b) => b.deref(),
        }
    }
}

impl<T: ?Sized> AsRef<T> for Bow<'_, T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T> Clone for Bow<'_, T>
where
    T: ?Sized + Clone,
{
    fn clone(&self) -> Self {
        match *self {
            Borrowed(b) => Borrowed(b),
            Boxed(ref b) => Boxed(b.clone()),
        }
    }
}

impl<T> PartialEq for Bow<'_, T>
where
    T: ?Sized + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&**self, &**other)
    }
}

impl<T: ?Sized + Eq> Eq for Bow<'_, T> {}

impl<T: ?Sized + Hash> Hash for Bow<'_, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&**self, state)
    }
}

impl<T> From<T> for Bow<'_, T> {
    fn from(t: T) -> Self {
        Bow::Boxed(t.into())
    }
}
