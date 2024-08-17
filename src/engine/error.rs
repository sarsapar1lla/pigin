use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct EngineError(String);

impl EngineError {
    pub fn new(message: impl Into<String>) -> Self {
        EngineError(message.into())
    }
}

impl Error for EngineError {}

impl Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
