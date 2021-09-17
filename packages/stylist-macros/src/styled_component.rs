// This file is borrowed from yew-macro/src/function_component.rs
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;

use super::styled_component_base::{styled_component_base_impl, StyledComponent};

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

pub fn styled_component_impl(
    name: StyledComponentName,
    component: StyledComponent,
) -> syn::Result<TokenStream> {
    let StyledComponentName { component_name } = name;

    let inner_tokens = styled_component_base_impl(component)?;

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
