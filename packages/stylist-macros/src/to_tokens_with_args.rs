use std::collections::HashMap;
use std::collections::HashSet;

use proc_macro2::{Literal, TokenStream};
use proc_macro_error::abort_call_site;
use quote::quote;

use stylist_core::ast::*;

use crate::argument::Argument;
use crate::fstring;

pub(crate) trait ToTokensWithArgs {
    fn to_tokens_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> TokenStream;
}

impl ToTokensWithArgs for Selector {
    fn to_tokens_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> TokenStream {
        let mut fragment_tokens = TokenStream::new();

        for frag in self.fragments.iter() {
            fragment_tokens.extend(frag.to_tokens_with_args(args, args_used));
        }

        quote! {
            ::stylist::ast::Selector{
                fragments: vec![#fragment_tokens].into(),
            }
        }
    }
}

impl ToTokensWithArgs for StyleAttribute {
    fn to_tokens_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> TokenStream {
        let key_s = Literal::string(&self.key);

        let mut val_tokens = TokenStream::new();

        for i in self.value.iter() {
            let current_tokens = i.to_tokens_with_args(args, args_used);

            val_tokens.extend(current_tokens);
        }
        quote! { ::stylist::ast::StyleAttribute{ key: #key_s.into(), value: vec![#val_tokens].into() } }
    }
}

impl ToTokensWithArgs for Block {
    fn to_tokens_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> TokenStream {
        let mut selector_tokens = TokenStream::new();

        for i in self.condition.iter() {
            let current_tokens = i.to_tokens_with_args(args, args_used);

            selector_tokens.extend(quote! {#current_tokens ,});
        }

        let mut style_attr_tokens = TokenStream::new();

        for i in self.style_attributes.iter() {
            let current_tokens = i.to_tokens_with_args(args, args_used);

            style_attr_tokens.extend(quote! {#current_tokens ,});
        }

        quote! {
            ::stylist::ast::Block {
                condition: vec![#selector_tokens].into(),
                style_attributes: vec![#style_attr_tokens].into(),
            }
        }
    }
}

impl ToTokensWithArgs for RuleContent {
    fn to_tokens_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> TokenStream {
        match self {
            Self::Block(ref m) => {
                let tokens = m.to_tokens_with_args(args, args_used);

                quote! { ::stylist::ast::RuleContent::Block(#tokens) }
            }
            Self::Rule(ref m) => {
                let tokens = m.to_tokens_with_args(args, args_used);

                quote! { ::stylist::ast::RuleContent::Rule(#tokens) }
            }
            Self::String(ref m) => {
                let s = Literal::string(m);
                quote! { ::stylist::ast::RuleContent::String(#s.into()) }
            }
        }
    }
}

impl ToTokensWithArgs for StringFragment {
    fn to_tokens_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> TokenStream {
        let fragments = match fstring::Parser::parse(&self.inner) {
            Ok(m) => m,
            Err(e) => abort_call_site!("{}", e),
        };

        let mut tokens = TokenStream::new();

        for frag in fragments.iter() {
            match frag {
                fstring::Fragment::Literal(ref m) => {
                    let s = Literal::string(m);

                    let current_tokens = quote! {
                        ::stylist::ast::StringFragment {
                            inner: #s.into(),
                        },
                    };

                    tokens.extend(current_tokens);
                }

                fstring::Fragment::Interpolation(ref m) => {
                    let arg = match args.get(m) {
                        Some(m) => m,
                        None => abort_call_site!("missing argument: {}", self.inner),
                    };

                    let arg_tokens = arg.tokens.clone();

                    args_used.insert(arg.name.clone());

                    let current_tokens = quote! {
                        ::stylist::ast::StringFragment {
                            inner: #arg_tokens.to_string().into(),
                        },
                    };

                    tokens.extend(current_tokens);
                }
            }
        }

        tokens
    }
}

impl ToTokensWithArgs for Rule {
    fn to_tokens_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> TokenStream {
        let mut cond_tokens = TokenStream::new();

        for i in self.condition.iter() {
            let current_tokens = i.to_tokens_with_args(args, args_used);

            cond_tokens.extend(current_tokens);
        }

        let mut content_tokens = TokenStream::new();

        for i in self.content.iter() {
            let current_tokens = i.to_tokens_with_args(args, args_used);

            content_tokens.extend(quote! {#current_tokens ,});
        }

        quote! {
            ::stylist::ast::Rule {
                condition: vec![#cond_tokens].into(),
                content: vec![#content_tokens].into(),
            }
        }
    }
}

impl ToTokensWithArgs for ScopeContent {
    fn to_tokens_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> TokenStream {
        match self {
            Self::Block(ref m) => {
                let tokens = m.to_tokens_with_args(args, args_used);

                quote! { ::stylist::ast::ScopeContent::Block(#tokens) }
            }
            Self::Rule(ref m) => {
                let tokens = m.to_tokens_with_args(args, args_used);

                quote! { ::stylist::ast::ScopeContent::Rule(#tokens) }
            }
        }
    }
}

impl ToTokensWithArgs for Sheet {
    fn to_tokens_with_args(
        &self,
        args: &HashMap<String, Argument>,
        args_used: &mut HashSet<String>,
    ) -> TokenStream {
        let mut scope_tokens = TokenStream::new();

        for i in self.iter() {
            let current_scope_tokens = i.to_tokens_with_args(args, args_used);

            scope_tokens.extend(quote! {#current_scope_tokens ,});
        }

        quote! { ::stylist::ast::Sheet::from(vec![#scope_tokens]) }
    }
}
