use convert_case::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::parse::{Lookahead1, Parse, ParseBuffer, Result as ParseResult};
use syn::{ext::IdentExt, token};
use syn::{
    punctuated::{Pair, Punctuated},
    Ident,
};

#[derive(Debug, Clone)]
pub struct DashedName {
    parts: Punctuated<Ident, token::Sub>,
}

impl DashedName {
    fn peek(lookahead: &Lookahead1) -> bool {
        lookahead.peek(Ident)
    }

    fn stringify(&self) -> String {
        self.parts
            .pairs()
            .map(|p| match p {
                Pair::Punctuated(t, _p) => format!("{}-", t.unraw()),
                Pair::End(t) => format!("{}", t.unraw()),
            })
            .collect()
    }
}

impl Parse for DashedName {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let parts = Punctuated::parse_separated_nonempty(input)?;
        Ok(Self { parts })
    }
}

#[derive(Debug, Clone)]
pub enum PunctuatedName {
    Simple {
        identifier: DashedName,
    },
    Dotted {
        dot: token::Dot,
        identifier: DashedName,
    },
    Dashed1 {
        dash1: token::Sub,
        identifier: DashedName,
    },
    Dashed2 {
        dash1: token::Sub,
        dash2: token::Sub,
        identifier: DashedName,
    },
    Hashed {
        pound: token::Pound,
        identifier: DashedName,
    },
}

impl PunctuatedName {
    pub fn peek(lookahead: &Lookahead1) -> bool {
        lookahead.peek(token::Dot)
            || lookahead.peek(token::Sub)
            || lookahead.peek(token::Pound)
            || DashedName::peek(lookahead)
    }

    pub fn quote(&self) -> TokenStream {
        let formatted = match self {
            Self::Simple { identifier } => identifier.stringify(),
            Self::Dashed1 { identifier, .. } => {
                format!("-{}", identifier.stringify())
            }
            Self::Dashed2 { identifier, .. } => {
                format!("--{}", identifier.stringify())
            }
            Self::Hashed { identifier, .. } => {
                format!("#{}", identifier.stringify())
            }
            Self::Dotted { identifier, .. } => {
                format!(".{}", identifier.stringify())
            }
        };

        quote! {
            #formatted
        }
    }
}

impl Parse for PunctuatedName {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        // !!Keep in sync with PunctuatedName#peek
        let lookahead1 = input.lookahead1();
        if lookahead1.peek(token::Dot) {
            let dot = input.parse()?;
            let identifier = input.parse()?;
            Ok(Self::Dotted { dot, identifier })
        } else if lookahead1.peek(token::Sub) {
            let dash1 = input.parse()?;
            let lookahead2 = input.lookahead1();
            if lookahead2.peek(token::Sub) {
                let dash2 = input.parse()?;
                let identifier = input.parse()?;
                Ok(Self::Dashed2 {
                    dash1,
                    dash2,
                    identifier,
                })
            } else if DashedName::peek(&lookahead2) {
                let identifier = input.parse()?;
                Ok(Self::Dashed1 { dash1, identifier })
            } else {
                Err(lookahead2.error())
            }
        } else if lookahead1.peek(token::Pound) {
            let pound = input.parse()?;
            let identifier = input.parse()?;
            Ok(Self::Hashed { pound, identifier })
        } else if DashedName::peek(&lookahead1) {
            let identifier = input.parse()?;
            Ok(Self::Simple { identifier })
        } else {
            Err(lookahead1.error())
        }
    }
}

#[derive(Debug, Clone)]
pub enum Identifier {
    Simple {
        identifier: DashedName,
    },
    Dashed1 {
        dash1: token::Sub,
        identifier: DashedName,
    },
    Dashed2 {
        dash1: token::Sub,
        dash2: token::Sub,
        identifier: DashedName,
    },
}

impl Parse for Identifier {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        // !!Keep in sync with Identifier#peek
        let lookahead1 = input.lookahead1();
        if lookahead1.peek(token::Sub) {
            let dash1 = input.parse()?;
            let lookahead2 = input.lookahead1();
            if lookahead2.peek(token::Sub) {
                let dash2 = input.parse()?;
                let identifier = input.parse()?;
                Ok(Self::Dashed2 {
                    dash1,
                    dash2,
                    identifier,
                })
            } else if DashedName::peek(&lookahead2) {
                let identifier = input.parse()?;
                Ok(Self::Dashed1 { dash1, identifier })
            } else {
                Err(lookahead2.error())
            }
        } else if DashedName::peek(&lookahead1) {
            let identifier = input.parse()?;
            Ok(Self::Simple { identifier })
        } else {
            Err(lookahead1.error())
        }
    }
}

impl Identifier {
    pub fn maybe_from_punctuated(punct: &PunctuatedName) -> Option<Self> {
        match punct.clone() {
            PunctuatedName::Dotted { .. } => None,
            PunctuatedName::Hashed { .. } => None,
            PunctuatedName::Simple { identifier } => Some(Self::Simple { identifier }),
            PunctuatedName::Dashed1 { dash1, identifier } => {
                Some(Self::Dashed1 { dash1, identifier })
            }
            PunctuatedName::Dashed2 {
                dash1,
                dash2,
                identifier,
            } => Some(Self::Dashed2 {
                dash1,
                dash2,
                identifier,
            }),
        }
    }

    fn stringify(&self) -> String {
        match self {
            Self::Simple { identifier } => identifier.stringify(),
            Self::Dashed1 { identifier, .. } => {
                format!("-{}", identifier.stringify())
            }
            Self::Dashed2 { identifier, .. } => {
                format!("--{}", identifier.stringify())
            }
        }
    }

    pub fn quote_at_rule(&self) -> TokenStream {
        let formatted = self.stringify().from_case(Case::Camel).to_case(Case::Kebab);
        quote! {
            #formatted
        }
    }

    pub fn quote_attribute(&self) -> TokenStream {
        let formatted = self.stringify().from_case(Case::Camel).to_case(Case::Kebab);
        quote! {
            #formatted
        }
    }
}

impl ToTokens for DashedName {
    fn to_tokens(&self, toks: &mut TokenStream) {
        self.parts.to_tokens(toks)
    }
}

impl ToTokens for PunctuatedName {
    fn to_tokens(&self, toks: &mut TokenStream) {
        match self {
            Self::Dashed1 { dash1, identifier } => {
                dash1.to_tokens(toks);
                identifier.to_tokens(toks);
            }
            Self::Dashed2 {
                dash1,
                dash2,
                identifier,
            } => {
                dash1.to_tokens(toks);
                dash2.to_tokens(toks);
                identifier.to_tokens(toks);
            }
            Self::Dotted { dot, identifier } => {
                dot.to_tokens(toks);
                identifier.to_tokens(toks);
            }
            Self::Hashed { pound, identifier } => {
                pound.to_tokens(toks);
                identifier.to_tokens(toks);
            }
            Self::Simple { identifier } => {
                identifier.to_tokens(toks);
            }
        }
    }
}

impl ToTokens for Identifier {
    fn to_tokens(&self, toks: &mut TokenStream) {
        match self {
            Self::Dashed1 { dash1, identifier } => {
                dash1.to_tokens(toks);
                identifier.to_tokens(toks);
            }
            Self::Dashed2 {
                dash1,
                dash2,
                identifier,
            } => {
                dash1.to_tokens(toks);
                dash2.to_tokens(toks);
                identifier.to_tokens(toks);
            }
            Self::Simple { identifier } => {
                identifier.to_tokens(toks);
            }
        }
    }
}
