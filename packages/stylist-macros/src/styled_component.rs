// This file is borrowed from yew-macro/src/function_component.rs
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Item, ItemFn};

use super::styled_component_impl::{styled_component_impl_impl, HookLike};

#[derive(Debug)]
pub enum StyledComponentArgs {
    Named { component_name: Ident },
    Empty,
}

impl Parse for StyledComponentArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self::Empty);
        }

        let component_name = input.parse()?;

        Ok(Self::Named { component_name })
    }
}

#[derive(Debug)]
pub struct StyledComponent {
    func: ItemFn,
}

impl Parse for StyledComponent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.parse()? {
            Item::Fn(func) => Ok(Self { func }),
            item => Err(syn::Error::new_spanned(
                item,
                "`styled_component` attribute can only be applied to functions",
            )),
        }
    }
}

pub fn styled_component_impl(
    args: StyledComponentArgs,
    component: StyledComponent,
) -> syn::Result<TokenStream> {
    let function_component_args = match args {
        StyledComponentArgs::Empty => quote! {},
        StyledComponentArgs::Named { component_name } => quote! { #component_name },
    };
    let StyledComponent { func } = component;

    let inner_tokens = styled_component_impl_impl(HookLike { func })?;

    Ok(quote! {
        #[::yew::functional::function_component( #function_component_args )]
        #inner_tokens
    })
}

pub fn macro_fn(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as StyledComponent);
    let attr = parse_macro_input!(attr as StyledComponentArgs);

    styled_component_impl(attr, item)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
