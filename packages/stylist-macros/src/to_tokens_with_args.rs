use std::collections::HashMap;

use proc_macro2::{Literal, TokenStream};
use quote::quote;

use stylist_core::ast::*;

use crate::argument::Argument;

pub(crate) trait ToTokensWithArgs {
    fn to_tokens_with_args(&self, args: &HashMap<String, Argument>) -> TokenStream;
}

impl ToTokensWithArgs for Selector {
    fn to_tokens_with_args(&self, _args: &HashMap<String, Argument>) -> TokenStream {
        let s = Literal::string(&self.inner);
        quote! { ::stylist::ast::Selector{ inner: #s.into() } }
    }
}

impl ToTokensWithArgs for StyleAttribute {
    fn to_tokens_with_args(&self, _args: &HashMap<String, Argument>) -> TokenStream {
        let key_s = Literal::string(&self.key);
        let value_s = Literal::string(&self.value);
        quote! { ::stylist::ast::StyleAttribute{ key: #key_s.into(), value: #value_s.into() } }
    }
}

impl ToTokensWithArgs for Block {
    fn to_tokens_with_args(&self, args: &HashMap<String, Argument>) -> TokenStream {
        let mut selector_tokens = TokenStream::new();

        for i in self.condition.iter() {
            let current_tokens = i.to_tokens_with_args(args);

            selector_tokens.extend(quote! {#current_tokens ,});
        }

        let mut style_attr_tokens = TokenStream::new();

        for i in self.style_attributes.iter() {
            let current_tokens = i.to_tokens_with_args(args);

            style_attr_tokens.extend(quote! {#current_tokens ,});
        }

        quote! {
            ::stylist::ast::Block {
                condition: ::std::borrow::Cow::Owned(vec![#selector_tokens]),
                style_attributes: ::std::borrow::Cow::Owned(vec![#style_attr_tokens])
            }
        }
    }
}

impl ToTokensWithArgs for RuleContent {
    fn to_tokens_with_args(&self, args: &HashMap<String, Argument>) -> TokenStream {
        match self {
            Self::Block(ref m) => {
                let tokens = m.to_tokens_with_args(args);

                quote! { ::stylist::ast::RuleContent::Block(#tokens) }
            }
            Self::Rule(ref m) => {
                let tokens = m.to_tokens_with_args(args);

                quote! { ::stylist::ast::RuleContent::Rule(#tokens) }
            }
            Self::String(ref m) => {
                let s = Literal::string(m);
                quote! { ::stylist::ast::RuleContent::String(#s.into()) }
            }
        }
    }
}

impl ToTokensWithArgs for Rule {
    fn to_tokens_with_args(&self, args: &HashMap<String, Argument>) -> TokenStream {
        let cond_s = Literal::string(&self.condition);

        let mut content_tokens = TokenStream::new();

        for i in self.content.iter() {
            let current_tokens = i.to_tokens_with_args(args);

            content_tokens.extend(quote! {#current_tokens ,});
        }

        quote! {
            ::stylist::ast::Rule {
                condition: ::std::borrow::Cow::Borrowed(#cond_s),
                content: ::std::borrow::Cow::Owned(vec![#content_tokens])
            }
        }
    }
}

impl ToTokensWithArgs for ScopeContent {
    fn to_tokens_with_args(&self, args: &HashMap<String, Argument>) -> TokenStream {
        match self {
            Self::Block(ref m) => {
                let tokens = m.to_tokens_with_args(args);

                quote! { ::stylist::ast::ScopeContent::Block(#tokens) }
            }
            Self::Rule(ref m) => {
                let tokens = m.to_tokens_with_args(args);

                quote! { ::stylist::ast::ScopeContent::Rule(#tokens) }
            }
        }
    }
}

impl ToTokensWithArgs for Sheet {
    fn to_tokens_with_args(&self, args: &HashMap<String, Argument>) -> TokenStream {
        let mut scope_tokens = TokenStream::new();

        for i in self.iter() {
            let current_scope_tokens = i.to_tokens_with_args(args);

            scope_tokens.extend(quote! {#current_scope_tokens ,});
        }

        quote! { ::stylist::ast::Sheet::from( ::std::borrow::Cow::Owned(vec![#scope_tokens])) }
    }
}
