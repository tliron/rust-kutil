use super::recipient::*;

use std::{cell::*, sync::*};

//
// ErrorRecipientRef
//

/// Common reference type for [ErrorRecipient].
pub type ErrorRecipientRef<'own, ErrorT> = Arc<RefCell<&'own mut dyn ErrorRecipient<ErrorT>>>;

/// Create a new [ErrorRecipientRef].
pub fn new_error_recipient_ref<ErrorT, ErrorRecipientT>(
    error_recipient: &mut ErrorRecipientT,
) -> ErrorRecipientRef<'_, ErrorT>
where
    ErrorRecipientT: ErrorRecipient<ErrorT>,
{
    ErrorRecipientRef::new(RefCell::new(error_recipient))
}

//
// ErrorRecipientToRef
//

/// Create an [ErrorRecipientRef].
pub trait ErrorRecipientToRef<'own, ErrorT, ErrorRecipientT> {
    /// Create an [ErrorRecipientRef].
    fn to_ref(&'own mut self) -> ErrorRecipientRef<'own, ErrorT>;
}

impl<'own, ErrorT, ErrorRecipientT> ErrorRecipientToRef<'own, ErrorT, ErrorRecipientT> for ErrorRecipientT
where
    ErrorRecipientT: ErrorRecipient<ErrorT>,
{
    fn to_ref(&'own mut self) -> ErrorRecipientRef<'own, ErrorT> {
        new_error_recipient_ref(self)
    }
}

//
// ErrorRecipientRefImplementation
//

/// An [ErrorRecipient] implementation for an [ErrorRecipientRef].
pub struct ErrorRecipientRefImplementation<'own, ErrorT> {
    /// Inner.
    pub inner: ErrorRecipientRef<'own, ErrorT>,
}

impl<'own, ErrorT> ErrorRecipient<ErrorT> for ErrorRecipientRefImplementation<'own, ErrorT> {
    fn give_error(&mut self, error: ErrorT) -> Result<(), ErrorT> {
        self.inner.borrow_mut().give_error(error)
    }
}

//
// ToErrorRecipient
//

/// Create an [ErrorRecipient] implementation for an [ErrorRecipientRef].
pub trait ToErrorRecipient<'own, ErrorT> {
    /// Create an [ErrorRecipient] implementation for an [ErrorRecipientRef].
    fn to_error_recipient(&self) -> ErrorRecipientRefImplementation<'own, ErrorT>;
}

impl<'own, ErrorT> ToErrorRecipient<'own, ErrorT> for ErrorRecipientRef<'own, ErrorT> {
    fn to_error_recipient(&self) -> ErrorRecipientRefImplementation<'own, ErrorT> {
        ErrorRecipientRefImplementation { inner: self.clone() }
    }
}
