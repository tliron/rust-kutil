//
// ErrorRecipient
//

/// A recipient of errors.
pub trait ErrorRecipient<ErrorT> {
    /// Gives an error to the recipient.
    ///
    /// Implementations may swallow the error (e.g. to accumulate it) or
    /// return it (fail-fast).
    fn give(&mut self, error: impl Into<ErrorT>) -> Result<(), ErrorT>;
}

//
// FailFastErrorRecipient
//

/// [ErrorRecipient] that fails on the first given error.
pub struct FailFastErrorRecipient;

impl<ErrorT> ErrorRecipient<ErrorT> for FailFastErrorRecipient {
    fn give(&mut self, error: impl Into<ErrorT>) -> Result<(), ErrorT> {
        Err(error.into())
    }
}
