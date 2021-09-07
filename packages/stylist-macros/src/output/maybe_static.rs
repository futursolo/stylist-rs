use super::{ContextRecorder, Reify};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub trait IntoCowVecTokens: IntoIterator
where
    Self::Item: Reify,
{
    // Get a TokenStream of an expression of type Cow<'_, [typ]>, containing
    // as elements the values formed by the expressions in this stream.
    // Depending on the context in which the expression can be expanded,
    // uses either Cow::Owned or Cow::Borrowed (currently always Cow::Owned).
    fn into_cow_vec_tokens(self, typ: TokenStream, ctx: &mut ContextRecorder) -> TokenStream;
}

impl<I> IntoCowVecTokens for I
where
    I: IntoIterator,
    I::Item: Reify,
{
    fn into_cow_vec_tokens(self, typ: TokenStream, ctx: &mut ContextRecorder) -> TokenStream {
        let mut inner_ctx = ContextRecorder::new();
        let contents: Vec<TokenStream> = self
            .into_iter()
            .map(|m| m.into_token_stream(&mut inner_ctx))
            .collect();

        if inner_ctx.is_const() {
            let name = Ident::new("items", Span::mixed_site());
            let content_len = contents.len();
            quote! {
                ::std::borrow::Cow::<[#typ]>::Borrowed ({
                    const #name: [#typ; #content_len] = [
                        #( #contents, )*
                    ];
                    &#name
                })
            }
        } else {
            ctx.uses_static(); // ::std::vec!
            ctx.uses_nested(&inner_ctx); // #contents
            quote! {
                ::std::borrow::Cow::<[#typ]>::Owned (
                    ::std::vec![
                        #( #contents, )*
                    ]
                )
            }
        }
    }
}
