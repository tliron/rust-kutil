use std::{error, fmt};

//
// Exit
//

/// Information on how to exit a program.
#[derive(Clone, Debug)]
pub struct Exit {
    /// Exit code.
    pub code: u8,

    /// Optional goodbye message.
    pub message: Option<String>,
}

impl Exit {
    /// Constructor.
    pub fn new(code: u8, message: Option<&str>) -> Self {
        let message = message.map(|message| message.into());
        Self { code, message }
    }

    /// Constructor.
    pub fn new_from<ToStringT>(code: u8, to_string: ToStringT) -> Self
    where
        ToStringT: ToString,
    {
        Self { code, message: Some(to_string.to_string()) }
    }

    /// Successful exit (code 0) without a message.
    pub fn success() -> Self {
        0.into()
    }
}

impl error::Error for Exit {}

impl fmt::Display for Exit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(message) => write!(formatter, "{}: {}", self.code, message),
            None => fmt::Display::fmt(&self.code, formatter),
        }
    }
}

// Conversions

impl From<u8> for Exit {
    fn from(value: u8) -> Self {
        Self::new(value, None)
    }
}

impl From<&str> for Exit {
    fn from(message: &str) -> Self {
        Self::new(1, Some(message))
    }
}

//
// HasExit
//

/// For types that can optionally have an [Exit].
pub trait HasExit: fmt::Display {
    /// Return the [Exit] if it exists.
    fn get_exit(&self) -> Option<&Exit>;
}
