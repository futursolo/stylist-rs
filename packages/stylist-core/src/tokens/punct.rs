use arcstr::Substr;

use super::{InputStr, InputTokens, Location, Token, TokenStream, TokenTree, Tokenize};

#[derive(Debug, Clone)]
pub struct Punct {
    inner: Substr,
    location: Location,
}

impl PartialEq for Punct {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Token for Punct {
    fn as_str(&self) -> &str {
        &self.inner
    }
    fn location(&self) -> &Location {
        &self.location
    }
}

impl Tokenize<InputStr> for Punct {
    fn tokenize(input: InputStr) -> Result<(TokenStream, InputStr), InputStr> {
        let valid_char = |c: &char| "#+,-.:;<@\\".contains(*c);

        if input.chars().next().filter(valid_char).is_some() {
            let (inner, location, rest) = input.split_at(1);

            Ok((TokenTree::Punct(Punct { inner, location }).into(), rest))
        } else {
            Err(input)
        }
    }
}

impl Tokenize<InputTokens> for Punct {
    fn tokenize(mut input: InputTokens) -> Result<(TokenStream, InputTokens), InputTokens> {
        use super::rtokens::*;

        if let Some(m) = input.get(0).cloned() {
            match m {
                RTokenTree::Punct(ref p) => {
                    input.pop_front();

                    let s = p.as_char().to_string();
                    let location = Location::Span(m.clone().into());

                    return Ok((
                        TokenTree::Punct(Punct {
                            inner: s.into(),
                            location,
                        })
                        .into(),
                        input,
                    ));
                }
                _ => (),
            }
        }

        Err(input)
    }
}
