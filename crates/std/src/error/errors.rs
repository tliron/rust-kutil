use super::recipient::*;

use std::{error::*, fmt, iter::*, slice, vec};

//
// Errors
//

/// An [Error] that contains zero or more errors.
///
/// Implements [ErrorRecipient] by accumulating errors.
#[derive(Debug, Clone)]
pub struct Errors<ErrorT> {
    /// The errors.
    pub errors: Vec<ErrorT>,
}

impl<ErrorT> Errors<ErrorT> {
    /// Constructor.
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// True if there are no errors.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Fails with self if there are errors.
    pub fn check(&self) -> Result<(), &Self> {
        if self.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl<ErrorT> ErrorRecipient<ErrorT> for Errors<ErrorT> {
    fn give(&mut self, error: impl Into<ErrorT>) -> Result<(), ErrorT> {
        self.errors.push(error.into());
        Ok(())
    }
}

impl<ErrorT> Error for Errors<ErrorT> where ErrorT: Error {}

impl<ErrorT> fmt::Display for Errors<ErrorT>
where
    ErrorT: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut i = self.errors.iter().peekable();
        while let Some(error) = i.next() {
            fmt::Display::fmt(error, formatter)?;
            if i.peek().is_some() {
                writeln!(formatter)?;
            }
        }
        Ok(())
    }
}

// Delegated

impl<ErrorT> IntoIterator for Errors<ErrorT> {
    type Item = ErrorT;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

impl<'own, ErrorT> IntoIterator for &'own Errors<ErrorT> {
    type Item = &'own ErrorT;
    type IntoIter = slice::Iter<'own, ErrorT>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.iter()
    }
}

impl<'own, ErrorT> IntoIterator for &'own mut Errors<ErrorT> {
    type Item = &'own mut ErrorT;
    type IntoIter = slice::IterMut<'own, ErrorT>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.iter_mut()
    }
}

// Conversions

impl<ErrorT> From<ErrorT> for Errors<ErrorT> {
    fn from(value: ErrorT) -> Self {
        let mut errors = Errors::new();
        errors.errors.push(value);
        errors
    }
}

impl<ErrorT> Into<Vec<ErrorT>> for Errors<ErrorT> {
    fn into(self) -> Vec<ErrorT> {
        self.errors
    }
}

//
// AsErrorsResult
//

/// Converts to a [Result] with [Errors].
pub trait AsErrorsResult<ReturnT, ErrorT> {
    /// Converts to a [Result] with [Errors].
    fn as_errors(self) -> Result<ReturnT, Errors<ErrorT>>;
}

impl<ReturnT, ErrorT> AsErrorsResult<ReturnT, ErrorT> for Result<ReturnT, ErrorT> {
    fn as_errors(self) -> Result<ReturnT, Errors<ErrorT>> {
        Ok(self?)
    }
}
