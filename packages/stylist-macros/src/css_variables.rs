use convert_case::{Case, Casing};
use proc_macro2::{Literal, Span, TokenStream};
use proc_macro_error::{abort, abort_call_site};
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Ident};

use crate::utils::get_rand_str;

pub(crate) fn macro_fn(input: DeriveInput) -> TokenStream {
    let data = match input.data {
        Data::Struct(ref m) => m.to_owned(),
        _ => abort_call_site!("only structs are supported!"),
    };

    let struct_ident = input.ident;

    let fields = match data.fields {
        Fields::Named(ref m) => m.named.iter().cloned().collect::<Vec<Field>>(),
        Fields::Unit => Vec::new(),
        Fields::Unnamed(_) => abort_call_site!("only named structs are supported at this moment!"),
    };

    let mut idents = Vec::new();
    let mut nested_idents = Vec::new();

    for field in fields {
        let ident = field.ident.unwrap();

        let mut css_var_attrs = field
            .attrs
            .iter()
            .filter(|m| m.path.get_ident().map(|m| m.to_string()).as_deref() == Some("css_vars"))
            .cloned();

        let css_var_attr = css_var_attrs.next();
        if let Some(m) = css_var_attrs.next() {
            abort!(m, "only 1 #[css_vars] attribute is allowed.");
        }

        // Either #[css_vars(nested)] or #[css_vars(skipped)]
        let kind = match css_var_attr {
            Some(m) => Some(match m.parse_args::<Ident>() {
                Ok(m) => m,
                Err(e) => return e.to_compile_error(),
            }),
            None => None,
        };

        let kind_str = kind.clone().map(|m| m.to_string());

        match (kind, kind_str.as_deref()) {
            (_, Some("nested")) => nested_idents.push(ident),
            (_, Some("skipped")) => {}
            (_, None) => idents.push(ident),
            (Some(m), Some(s)) => abort!(m, "unknown kind: {}", s),
            _ => unreachable!(),
        }
    }

    let entropy = Literal::string(&get_rand_str());

    let ident_strs = idents
        .iter()
        .map(|m| Literal::string(&m.to_string().to_case(Case::Kebab)))
        .collect::<Vec<_>>();

    let w_ident = Ident::new("_w", Span::mixed_site());

    let mut ident_inserts = TokenStream::new();
    for (ident, ident_str) in idents.iter().zip(ident_strs.iter()) {
        let stmt = quote! {
            ::std::collections::HashMap::insert(
                #w_ident,
                ::std::format!(
                    "--stylist-{}-{}",
                    ::stylist::CssVariables::entropy(self),
                    #ident_str
                ),
                ::std::string::ToString::to_string(&self.#ident),
            );
        };

        ident_inserts.extend(stmt);
    }

    for nested_ident in nested_idents.iter() {
        let stmt = quote! {
            {
                ::stylist::CssVariables::__append(
                    &self.#nested_ident,
                    #w_ident,
                );
            }
        };

        ident_inserts.extend(stmt);
    }

    quote! {
        #[automatically_derived]
        impl ::stylist::CssVariables for #struct_ident {
            fn entropy(&self) -> &'static ::std::primitive::str {
                #entropy
            }

            fn __append(&self, #w_ident: &mut ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
                #ident_inserts
            }
        }
    }
}
