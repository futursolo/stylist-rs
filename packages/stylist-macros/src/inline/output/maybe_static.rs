use super::{AllowedUsage, ContextRecorder, Reify};
use proc_macro2::TokenStream;
use quote::quote;
use std::iter::FromIterator;

pub trait IntoCowVecTokens: IntoIterator
where
    Self::Item: Reify,
{
    // Get a TokenStream of an expression of type Cow<'_, [typ]>, containing
    // as elements the values formed by the expressions in this stream.
    // Depending on the context in which the expression can be expanded,
    // uses either Cow::Owned or Cow::Borrowed (currently always Cow::Owned).
    fn into_cow_vec_tokens(self, ctx: &mut ContextRecorder) -> TokenStream;
}

impl<I> IntoCowVecTokens for I
where
    I: IntoIterator,
    I::Item: Reify,
{
    fn into_cow_vec_tokens(self, ctx: &mut ContextRecorder) -> TokenStream {
        self.into_iter()
            .map(|e| e.into_context_aware_tokens())
            .collect::<MaybeStatic<_>>()
            .into_cow_vec_tokens_impl(ctx)
    }
}

// Used e.g. to decide whether a sheet can be statically cached in a Lazy or must be
// created everytime anew.
pub struct MaybeStatic<T> {
    value: T,
    context: ContextRecorder,
}

impl<T> MaybeStatic<T> {
    pub fn in_context(value: T, context: ContextRecorder) -> Self {
        Self { value, context }
    }
    pub fn into_value(self) -> (T, AllowedUsage) {
        (self.value, self.context.usage())
    }
}

impl MaybeStatic<Vec<TokenStream>> {
    // Get a TokenStream of an expression of type Cow<'_, [typ]>, containing
    // as elements the expressions form from the Vec<_> in this MaybeStatic.
    // Depending on the context in which the expression can be expanded,
    // uses either Cow::Owned or Cow::Borrowed (currently always Cow::Owned).
    fn into_cow_vec_tokens_impl(self, ctx: &mut ContextRecorder) -> TokenStream {
        ctx.merge_with(&self.context);
        let contents = self.value;
        quote! {
            ::std::vec![
                #( #contents, )*
            ].into()
        }
    }
}

// compare to the implementation of Result<V, E>: FromIterator + Extend
struct StaticShun<'a, I> {
    iter: I,
    context: &'a mut ContextRecorder,
}

impl<'a, T, I: Iterator<Item = MaybeStatic<T>>> Iterator for StaticShun<'a, I> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self.iter.next() {
            None => None,
            Some(MaybeStatic { value, context }) => {
                self.context.merge_with(&context);
                Some(value)
            }
        }
    }
}

impl<T, V: FromIterator<T>> FromIterator<MaybeStatic<T>> for MaybeStatic<V> {
    fn from_iter<I: IntoIterator<Item = MaybeStatic<T>>>(iter: I) -> Self {
        let mut context = Default::default();
        let v = StaticShun {
            iter: iter.into_iter(),
            context: &mut context,
        }
        .collect();
        MaybeStatic::in_context(v, context)
    }
}
