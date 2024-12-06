use std::fmt;

//
// Exit
//

#[derive(Debug)]
pub struct Exit {
    pub code: u8,
    pub message: Option<String>,
}

impl Exit {
    pub fn new(code: u8, message: Option<String>) -> Self {
        Self { code, message }
    }

    pub fn success() -> Self {
        0.into()
    }
}

impl From<u8> for Exit {
    fn from(value: u8) -> Self {
        Self::new(value, None)
    }
}

impl fmt::Display for Exit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(message) => write!(formatter, "{}: {}", self.code, message),
            None => self.code.fmt(formatter),
        }
    }
}

//
// HasExit
//

pub trait HasExit {
    fn get_exit(&self) -> Option<&Exit>;
}
