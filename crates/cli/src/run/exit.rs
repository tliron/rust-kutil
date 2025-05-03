use std::{error, fmt};

//
// Exit
//

/// Information on how to exit a program.
#[derive(Debug)]
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

//
// HasExit
//

/// For types that can optionally have an [Exit].
pub trait HasExit: fmt::Display {
    /// Return the [Exit] if it exists.
    fn get_exit(&self) -> Option<&Exit>;
}
