use super::{ContextRecorder, Reify};
use proc_macro2::TokenStream;
use quote::quote;

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
        let contents: Vec<TokenStream> =
            self.into_iter().map(|m| m.into_token_stream(ctx)).collect();

        quote! {
            ::std::vec![
                #( #contents, )*
            ].into()
        }

        // In the future, if there's a need to collect sub contexts for optimisation:
        // let tokens = TokenStream::new();
        // let mut ctx = ContextRecorder::new();
        // for i in self.into_iter() {
        //     tokens.extend(i.into_token_stream(&mut ctx));
        //     tokens.extend(quote! {, });
        // }
        // quote! { ::std::vec![#items] }
    }
}
