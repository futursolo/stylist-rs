use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::abort;
use quote::quote;
use syn::{ExprField, LitStr, Member};

pub(crate) fn macro_fn(input: TokenStream) -> TokenStream {
    let field: ExprField = match syn::parse2(input) {
        Ok(m) => m,
        Err(e) => return e.into_compile_error(),
    };

    let field_ident = match field.member {
        Member::Named(ref m) => m,
        Member::Unnamed(_) => abort!(field, "unnamed fields are not supported!"),
    };

    let struct_var_ident = Ident::new("_var", Span::mixed_site());
    let struct_var = field.base;
    let struct_attrs = field.attrs;

    let field_str = LitStr::new(
        &field_ident.to_string().to_case(Case::Kebab),
        Span::mixed_site(),
    );

    quote! {
        #(#[#struct_attrs])*
        {
            let #struct_var_ident = #struct_var;

            format!(
                "var(--stylist-{}-{}, {})",
                ::stylist::CssVariables::entropy(&#struct_var_ident),
                #field_str,
                #struct_var_ident.#field_ident,
            )
        }
    }
}
