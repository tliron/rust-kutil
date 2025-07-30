use super::{accumulator::*, recipient::*};

//
// FailFastErrorRecipient
//

/// [ErrorRecipient] that fails on the first given error.
pub struct FailFastErrorRecipient;

impl FailFastErrorRecipient {
    /// Creates an fail-fast [ErrorAccumulator].
    pub fn as_accumulator<ErrorT>(&mut self) -> ErrorAccumulator<'_, ErrorT> {
        ErrorAccumulator::FailFast
    }
}

impl<ErrorT> ErrorRecipient<ErrorT> for FailFastErrorRecipient {
    fn give(&mut self, error: impl Into<ErrorT>) -> Result<(), ErrorT> {
        Err(error.into())
    }
}
