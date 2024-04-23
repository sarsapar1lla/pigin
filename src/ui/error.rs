use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct UiError(String);

impl UiError {
    pub fn new(message: impl Into<String>) -> Self {
        UiError(message.into())
    }
}

impl Error for UiError {}

impl Display for UiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
