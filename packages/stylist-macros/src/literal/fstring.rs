use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_while};
use nom::character::complete::{alpha1, alphanumeric1};
use nom::combinator::{all_consuming, cut, map, opt, recognize};
use nom::error::{context, convert_error, ErrorKind, ParseError, VerboseError};
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use stylist_core::{Error, Result};

#[cfg(test)]
use log::trace;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Fragment {
    Literal(String),
    Interpolation(String),
}

#[derive(Debug)]
pub(crate) struct Parser {}

impl Parser {
    /// Returns Error when string is Empty
    fn expect_non_empty(i: &str) -> std::result::Result<(), nom::Err<VerboseError<&str>>> {
        if i.is_empty() {
            Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::LengthValue,
            )))
        } else {
            Ok(())
        }
    }

    /// Parse whitespace
    fn sp(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
        Self::expect_non_empty(i)?;

        let chars = " \t\r\n";
        context("Whitespace", take_while(move |c| chars.contains(c)))(i)
    }

    /// Drop whitespaces
    fn trimmed<'a, F, O>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, VerboseError<&str>>
    where
        F: nom::Parser<&'a str, O, VerboseError<&'a str>>,
    {
        context(
            "Trimmed",
            delimited(
                // Drop Preceeding whitespaces.
                opt(Self::sp),
                // Parse until finishes
                f,
                // Drop Trailing whitespaces.
                opt(Self::sp),
            ),
        )
    }

    /// Parse a string interpolation.
    fn interpolation(i: &str) -> IResult<&str, Fragment, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Interpolation: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "Interpolation",
            map(
                delimited(
                    tag("${"),
                    context(
                        "ArgumentName",
                        cut(Self::trimmed(recognize(preceded(
                            alpha1,
                            many0(alt((alphanumeric1, tag("_")))),
                        )))),
                    ),
                    tag("}"),
                ),
                |p: &str| Fragment::Interpolation(p.to_string()),
            ),
        )(i);

        #[cfg(test)]
        trace!("Interpolation: {:#?}", result);

        result
    }

    #[allow(clippy::let_and_return)]
    fn literal(i: &str) -> IResult<&str, Fragment, VerboseError<&str>> {
        #[cfg(test)]
        trace!("Literal: {}", i);

        Self::expect_non_empty(i)?;

        let result = context(
            "Literal",
            map(is_not("${"), |m: &str| Fragment::Literal(m.to_string())),
        )(i);

        #[cfg(test)]
        trace!("Literal: {:#?}", result);

        result
    }

    fn fragments(i: &str) -> IResult<&str, Vec<Fragment>, VerboseError<&str>> {
        all_consuming(many0(alt((
            // match escape sequence first.
            map(tag("$${"), |_m: &str| Fragment::Literal("${".to_string())),
            Self::literal,
            Self::interpolation,
        ))))(i)
    }

    pub fn parse(s: &str) -> Result<Vec<Fragment>> {
        match Self::fragments(s) {
            // Converting to String, primarily due to lifetime requirements.
            Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(Error::Parse {
                reason: convert_error(s, e.clone()),
                source: Some(VerboseError {
                    errors: e
                        .errors
                        .into_iter()
                        .map(|(i, e)| (i.to_string(), e))
                        .collect(),
                }),
            }),
            Err(nom::Err::Incomplete(e)) => Err(Error::Parse {
                reason: format!("{e:#?}"),
                source: None,
            }),
            Ok((_, res)) => Ok(res),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_simple() -> Result<()> {
        init();
        let parsed = Parser::parse("simple string")?;

        let expected = vec![Fragment::Literal("simple string".to_string())];

        assert_eq!(parsed, expected);

        Ok(())
    }

    #[test]
    fn test_complex() -> Result<()> {
        init();
        let parsed = Parser::parse("not so ${simple} string")?;

        let expected = vec![
            Fragment::Literal("not so ".to_string()),
            Fragment::Interpolation("simple".to_string()),
            Fragment::Literal(" string".to_string()),
        ];

        assert_eq!(parsed, expected);

        Ok(())
    }

    #[test]
    fn test_invalid() {
        init();
        let result = Parser::parse("${1nvalid} string");

        log::debug!("{:#?}", result);

        assert!(result.is_err());
    }
}
