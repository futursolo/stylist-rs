use crate::output::OutputFragment;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    braced,
    parse::{Parse, ParseBuffer, Result as ParseResult},
    token, Expr,
};

#[derive(Debug, Clone)]
pub struct InterpolatedExpression {
    dollar: token::Dollar,
    braces: token::Brace,
    expr: Box<Expr>,
}

impl ToTokens for InterpolatedExpression {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.dollar.to_tokens(tokens);
        self.braces.surround(tokens, |toks| {
            self.expr.to_tokens(toks);
        });
    }
}

impl Parse for InterpolatedExpression {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let dollar = input.parse()?;
        let inner;
        let braces = braced!(inner in input);
        let expr = Box::new(inner.parse()?);
        Ok(InterpolatedExpression {
            dollar,
            braces,
            expr,
        })
    }
}

impl InterpolatedExpression {
    pub fn to_output_fragment(&self) -> OutputFragment {
        (&*self.expr).into()
    }
}
