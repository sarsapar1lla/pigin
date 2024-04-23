use std::error::Error;
use std::fmt::Display;

use nom::error::ErrorKind;
use nom::error::ParseError;

#[derive(Debug)]
pub struct PgnParseError(String);

impl PgnParseError {
    pub fn new(message: impl Into<String>) -> Self {
        PgnParseError(message.into())
    }

    pub fn message(&self) -> &str {
        &self.0
    }
}

impl ParseError<&str> for PgnParseError {
    fn append(input: &str, _kind: ErrorKind, other: Self) -> Self {
        let message = format!("Parsing input '{input}' failed: '{}'", other.0);
        PgnParseError(message)
    }

    fn from_char(input: &str, _: char) -> Self {
        let message = format!("Parsing input '{input}' failed");
        PgnParseError(message)
    }

    fn from_error_kind(input: &str, kind: ErrorKind) -> Self {
        let message = format!("Parsing input '{input}' failed: '{kind:?}'");
        PgnParseError(message)
    }

    fn or(self, _other: Self) -> Self {
        self
    }
}

impl Error for PgnParseError {}

impl Display for PgnParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
