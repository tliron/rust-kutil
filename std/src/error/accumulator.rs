use super::{errors::*, recipient::*};

//
// ErrorAccumulator
//

/// An [ErrorRecipient] that can either fail-fast or accumulate.
///
/// Note that unlike [ErrorRecipient], which is a trait, this is a struct and as such has only its
/// defined behavior. To allow for more flexibility follow the example in [ErrorRecipient] instead
/// of using this type.
pub enum ErrorAccumulator<'errors, ErrorT> {
    /// Fail on the first given error.
    FailFast,

    /// Accumulate errors.
    Errors(&'errors mut Errors<ErrorT>),
}

impl<'errors, ErrorT> ErrorAccumulator<'errors, ErrorT> {
    /// Constructor.
    pub fn new(errors: &'errors mut Errors<ErrorT>) -> Self {
        Self::Errors(errors)
    }
}

impl<'errors, ErrorT> ErrorRecipient<ErrorT> for ErrorAccumulator<'errors, ErrorT> {
    fn give(&mut self, error: impl Into<ErrorT>) -> Result<(), ErrorT> {
        match self {
            Self::FailFast => Err(error.into()),
            Self::Errors(errors) => errors.give(error),
        }
    }
}
