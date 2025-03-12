use std::{error::Error, fmt::Display};


#[derive(Debug)]
pub struct MainError {
   pub message: String,
}

impl Display for MainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl Error for MainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        &self.message
    }

    /// Returns the underlying cause of this error, if any.
    ///
    /// This method delegates to the `source` method to provide compatibility
    /// with older error APIs that use `cause`.
    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}