// This file is borrowed from yew-macro/src/function_component.rs
use std::iter::FromIterator;

use proc_macro2::{Group, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::{Ident, Item, ItemFn};

#[derive(Debug)]
pub struct StyledComponent {
    func: ItemFn,
}

impl Parse for StyledComponent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed: Item = input.parse()?;

        match parsed {
            Item::Fn(func) => Ok(Self { func }),
            item => Err(syn::Error::new_spanned(
                item,
                "`styled_component` attribute can only be applied to functions",
            )),
        }
    }
}

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

#[derive(Debug, PartialEq)]
enum State {
    NotFound,
    Ident,
    Excl,
}

pub fn affix_manager(macro_name: &str, input: TokenStream, mgr_ident: Ident) -> TokenStream {
    let tokens = input.into_iter();

    let mut completed: Vec<TokenTree> = Vec::new();

    let mut state = State::NotFound;

    for token in tokens {
        match token {
            TokenTree::Ident(ref m) => {
                if state == State::NotFound && *m == macro_name {
                    state = State::Ident;
                } else {
                    state = State::NotFound;
                }

                completed.push(token);
            }

            TokenTree::Punct(ref m) => {
                if state == State::Ident && m.as_char() == '!' {
                    state = State::Excl;
                } else {
                    state = State::NotFound;
                }

                completed.push(token);
            }
            TokenTree::Literal(_) => {
                state = State::NotFound;
                completed.push(token);
            }
            TokenTree::Group(ref m) => {
                let content = affix_manager(macro_name, m.stream(), mgr_ident.clone());
                let group_tokens = Group::new(m.delimiter(), content).to_token_stream();

                completed.extend(group_tokens);

                if state == State::Excl {
                    completed.extend(quote! {
                        .with_manager({
                            #![allow(clippy::redundant_clone)]
                            #mgr_ident.clone()
                        })
                    });
                }

                state = State::NotFound;
            }
        }
    }

    TokenStream::from_iter(completed)
}

pub fn styled_component_impl(
    name: StyledComponentName,
    component: StyledComponent,
) -> syn::Result<TokenStream> {
    let StyledComponentName { component_name } = name;

    let StyledComponent { func } = component;

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = func;

    // Turn into token stream, so it's easier to process.
    let block_tokens = block.to_token_stream();

    let mgr_ident = Ident::new("__stylist_style_manager__", Span::mixed_site());
    let justified_tokens = affix_manager("css", block_tokens, mgr_ident.clone());

    let quoted = quote! {
        #(#attrs)*
        #[::yew::functional::function_component(#component_name)]
        #vis #sig {
            let #mgr_ident = ::yew::functional::use_context::<::stylist::manager::StyleManager>().unwrap_or_default();
            #[allow(unused_imports)]
            use ::stylist::css;

            #justified_tokens
        }
    };

    Ok(quoted)
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
