#[derive(Debug)]
pub struct UiError(String);

impl UiError {
    pub fn new(message: impl Into<String>) -> Self {
        UiError(message.into())
    }
}
