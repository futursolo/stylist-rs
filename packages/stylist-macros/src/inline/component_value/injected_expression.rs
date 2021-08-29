use super::super::output::OutputFragment;
use proc_macro2::{Span, TokenStream};
use quote::{quote_spanned, ToTokens};
use syn::{
    braced,
    parse::{Parse, ParseBuffer, Result as ParseResult},
    token, Expr, ExprLit, Ident, Lit,
};

#[derive(Debug, Clone)]
pub struct InjectedExpression {
    dollar: token::Dollar,
    braces: token::Brace,
    expr: Box<Expr>,
}

impl ToTokens for InjectedExpression {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.dollar.to_tokens(tokens);
        self.braces.surround(tokens, |toks| {
            self.expr.to_tokens(toks);
        });
    }
}

impl Parse for InjectedExpression {
    fn parse(input: &ParseBuffer) -> ParseResult<Self> {
        let dollar = input.parse()?;
        let inner;
        let braces = braced!(inner in input);
        let expr = Box::new(inner.parse()?);
        Ok(InjectedExpression {
            dollar,
            braces,
            expr,
        })
    }
}

impl InjectedExpression {
    pub fn to_output_fragment(&self) -> OutputFragment {
        let injected = &self.expr;
        if let Expr::Lit(ExprLit {
            lit: Lit::Str(ref litstr),
            ..
        }) = **injected
        {
            return OutputFragment::Str(litstr.clone());
        }

        let ident_result = Ident::new("expr", Span::mixed_site());
        let ident_write_expr = Ident::new("write_expr", Span::mixed_site());
        // quote spanned here so that errors related to calling #ident_write_expr show correctly
        let quoted = quote_spanned! {self.braces.span=>
            {
                fn #ident_write_expr<V: ::std::fmt::Display>(v: V) -> ::std::string::String {
                    use ::std::fmt::Write;
                    let mut #ident_result = ::std::string::String::new();
                    ::std::write!(&mut #ident_result, "{}", v).expect("");
                    #ident_result
                }
                #ident_write_expr(#injected).into()
            }
        };
        OutputFragment::Raw(quoted)
    }
}
