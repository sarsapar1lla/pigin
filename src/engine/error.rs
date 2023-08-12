#[derive(Debug)]
pub struct EngineError(String);

impl EngineError {
    pub fn new(message: impl Into<String>) -> Self {
        EngineError(message.into())
    }
}
