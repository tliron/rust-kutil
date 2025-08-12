use super::context::*;

use std::io::*;

const TO_STRING_BUFFER_CAPACITY: usize = 1024;

//
// Depict
//

/// Depict the object in a manner suitable for terminals.
///
/// May include colors and styles.
pub trait Depict {
    /// Write a depiction suitable for terminals.
    ///
    /// Required behavior for implementations:
    ///
    /// 1. Depictions *must not* end in a newline.
    /// 2. If *not* starting with a newline and *not* empty, *must* call [DepictionContext::separate] first.
    /// 3. All lines *after* the first (but *not* the first) *must* start with the [DepictionContext] indentation.
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> Result<()>
    where
        WriteT: Write;

    /// Write the depiction with a final newline.
    fn write_depiction<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> Result<()>
    where
        WriteT: Write,
    {
        self.depict(writer, context)?;
        writeln!(writer)
    }

    /// Print the depiction to [anstream::stdout] with a final newline.
    ///
    /// Panics on write [Error].
    fn print_depiction(&self, context: &DepictionContext) {
        self.write_depiction(&mut anstream::stdout(), context).expect("write_depiction");
    }

    /// Print the depiction to [anstream::stderr] with a final newline.
    ///
    /// Panics on write [Error].
    fn eprint_depiction(&self, context: &DepictionContext) {
        self.write_depiction(&mut anstream::stderr(), context).expect("write_depiction");
    }

    /// Capture [depict](Depict::depict) into a [String].
    fn to_depiction(&self, context: &DepictionContext) -> Result<String> {
        let mut writer = Vec::with_capacity(TO_STRING_BUFFER_CAPACITY);
        self.depict(&mut writer, context)?;
        String::from_utf8(writer.into()).map_err(Error::other)
    }
}
