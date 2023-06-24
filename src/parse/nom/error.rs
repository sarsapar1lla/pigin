use nom::error::ErrorKind;
use nom::error::ParseError;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    Internal(String),
    Nom(ErrorKind),
}

impl ParseError<&str> for ParserError {
    fn from_error_kind(input: &str, kind: ErrorKind) -> Self {
        ParserError::Nom(kind)
    }

    fn append(_: &str, _: ErrorKind, other: Self) -> Self {
        other
    }
}
