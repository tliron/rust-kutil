use std::fmt;

//
// ParseError
//

/// Parse error.
#[derive(Debug)]
pub struct ParseError(Option<String>);

impl ParseError {
    /// Constructor.
    pub fn new(message: Option<String>) -> Self {
        Self(message)
    }
}

impl From<String> for ParseError {
    fn from(message: String) -> Self {
        Self::new(Some(message))
    }
}

impl From<&str> for ParseError {
    fn from(message: &str) -> Self {
        Self::new(Some(message.into()))
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(message) => write!(formatter, "could not parse: {}", message),
            None => fmt::Display::fmt("could not parse", formatter),
        }
    }
}
