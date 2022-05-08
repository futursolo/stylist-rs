// This file is borrowed from yew-macro/src/function_component.rs
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Item, ItemFn};

use super::styled_component_impl::{styled_component_impl_impl, HookLike};

#[derive(Debug)]
pub struct StyledComponentName {
    component_name: Ident,
}

impl Parse for StyledComponentName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Err(input.error("expected identifier for the component"));
        }

        let component_name = input.parse()?;

        Ok(Self { component_name })
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
    name: StyledComponentName,
    component: StyledComponent,
) -> syn::Result<TokenStream> {
    let StyledComponentName { component_name } = name;
    let StyledComponent { func } = component;

    let inner_tokens = styled_component_impl_impl(HookLike { func })?;

    Ok(quote! {
        #[::yew::functional::function_component(#component_name)]
        #inner_tokens
    })
}

pub fn macro_fn(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as StyledComponent);
    let attr = parse_macro_input!(attr as StyledComponentName);

    styled_component_impl(attr, item)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
