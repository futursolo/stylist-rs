//! This module implements a type abstractly tracking in what kind of expression context
//! an item appears. This information is leverage to provide improved performance and
//! static caching of parts of the generated output.
use std::{
    iter::FromIterator,
    ops::{BitAnd, BitAndAssign},
};

use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExpressionContext {
    Dynamic,
    Static,
    // TODO: we can probably avoid a few allocations if we track which parts
    // of the ast can be constructed statically (with const methods).
    // Keep in mind to change Self::MAX and adjust the generation of cow-vec tokens.
    // Const,
}

impl ExpressionContext {
    const MAX: Self = Self::Static;
}

impl BitAnd for ExpressionContext {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.min(rhs)
    }
}

impl BitAndAssign for ExpressionContext {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

// Used to decide whether a sheet can be statically cached or must be
// created everytime anew.
pub struct MaybeStatic<T> {
    value: T,
    context: ExpressionContext,
}

impl<T> MaybeStatic<T> {
    pub fn in_context(context: ExpressionContext, value: T) -> Self {
        Self { value, context }
    }
    pub fn statick(value: T) -> Self {
        Self::in_context(ExpressionContext::Static, value)
    }
    pub fn dynamic(value: T) -> Self {
        Self::in_context(ExpressionContext::Dynamic, value)
    }
    pub fn into_value(self) -> (T, ExpressionContext) {
        (self.value, self.context)
    }
    pub fn flat_map<V>(self, f: impl FnOnce(T) -> MaybeStatic<V>) -> MaybeStatic<V> {
        let mapped = f(self.value);
        MaybeStatic {
            value: mapped.value,
            context: self.context & mapped.context,
        }
    }
}

impl MaybeStatic<Vec<TokenStream>> {
    pub fn into_cow_vec_tokens(self, typ: TokenStream) -> MaybeStatic<TokenStream> {
        let contents = self.value.into_iter();
        let quoted = quote! {
            ::std::borrow::Cow::<'_, [#typ]>::Owned(
                ::std::vec![
                    #( #contents, )*
                ]
            )
        };
        // using the vec![] macro means this is at best static context, not const
        MaybeStatic::in_context(self.context & ExpressionContext::Static, quoted)
        // if self.can_be_const {
        //     let len = contents.len();
        //     let ident = Ident::new("v", Span::mixed_site());
        //     MaybeStatic::konst(quote! {
        //         {
        //             static #ident: [#typ; #len] = [
        //                 #( #contents, )*
        //             ];
        //             &#ident.into()
        //         }
        //     })
        // }
    }
}

// compare to the implementation of Result<V, E>: FromIterator + Extend
struct StaticShun<'a, I> {
    iter: I,
    context: &'a mut ExpressionContext,
}

impl<'a, T, I: Iterator<Item = MaybeStatic<T>>> Iterator for StaticShun<'a, I> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self.iter.next() {
            None => None,
            Some(MaybeStatic { value, context }) => {
                *self.context &= context;
                Some(value)
            }
        }
    }
}

impl<T, V: FromIterator<T>> FromIterator<MaybeStatic<T>> for MaybeStatic<V> {
    fn from_iter<I: IntoIterator<Item = MaybeStatic<T>>>(iter: I) -> Self {
        let mut context = ExpressionContext::MAX;
        let v = StaticShun {
            iter: iter.into_iter(),
            context: &mut context,
        }
        .collect();
        MaybeStatic::in_context(context, v)
    }
}

impl<A, T: Extend<A>> Extend<MaybeStatic<A>> for MaybeStatic<T> {
    fn extend<I: IntoIterator<Item = MaybeStatic<A>>>(&mut self, iter: I) {
        let mut extend_context = ExpressionContext::MAX;
        self.value.extend(StaticShun {
            iter: iter.into_iter(),
            context: &mut extend_context,
        });
        self.context &= extend_context;
    }
}
