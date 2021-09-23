use proc_macro2 as r;

#[derive(Debug, Clone)]
pub enum Fragment {
    Literal(String),
    Expr(r::TokenStream),
}
