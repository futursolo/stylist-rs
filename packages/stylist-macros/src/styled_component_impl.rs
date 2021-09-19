// This file is borrowed from yew-macro/src/function_component.rs
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::{Ident, Item, ItemFn};

#[derive(Debug)]
pub struct HookLike {
    pub func: ItemFn,
}

impl Parse for HookLike {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.parse()? {
            Item::Fn(func) => Ok(Self { func }),
            item => Err(syn::Error::new_spanned(
                item,
                "`styled_component_impl` attribute can only be applied to functions",
            )),
        }
    }
}

#[derive(Debug)]
pub struct StyledComponentBaseArgs;

impl Parse for StyledComponentBaseArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if !input.is_empty() {
            return Err(input.error("unexpected arguments to styled_component_impl"));
        }
        Ok(StyledComponentBaseArgs)
    }
}

pub fn styled_component_impl_impl(item: HookLike) -> syn::Result<TokenStream> {
    let HookLike { func } = item;

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = func;

    let mgr_ident = Ident::new("__stylist_style_manager__", Span::mixed_site());
    let macro_tokens = quote! {
        macro_rules! css {
            ($( $args:tt )*) => {
                ::stylist::css!($($args)*).with_manager({
                    #[allow(clippy::redundant_clone)]
                    #mgr_ident.clone()
                })
            }
        }
    };

    let quoted = quote! {
        #(#attrs)*
        #vis #sig {
            let #mgr_ident = ::yew::functional::use_context::<::stylist::manager::StyleManager>().unwrap_or_default();
            #macro_tokens

            #block
        }
    };

    Ok(quoted)
}

pub fn macro_fn(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as HookLike);
    let _ = parse_macro_input!(attr as StyledComponentBaseArgs);

    styled_component_impl_impl(item)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
