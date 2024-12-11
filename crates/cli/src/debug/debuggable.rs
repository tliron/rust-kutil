use super::{prefix::*, theme::*};

use std::io::*;

//
// Debuggable
//

/// Can write a debug representation of itself that can be indented, styled, and multiline.
///
/// Designed to support complex nested hierarchies.
pub trait Debuggable {
    /// Write the debug representation.
    ///
    /// Required behavior for implementations:
    ///
    /// 1. Representations *must not* end in a newline.
    /// 2. All lines *after* the first (but *not* the first) *must* start with the provided prefix.
    fn write_debug_representation<WriteT>(
        &self,
        writer: &mut WriteT,
        prefix: &DebugPrefix,
        theme: &Theme,
    ) -> Result<()>
    where
        WriteT: Write;

    /// Write the debug representation with the default theme and a final newline.
    fn write_debug<WriteT>(&self, writer: &mut WriteT) -> Result<()>
    where
        WriteT: Write,
    {
        let theme = Theme::default();
        self.write_debug_representation(writer, &DebugPrefix::new(theme.delimiter), &theme)?;
        writeln!(writer)
    }

    /// Write the debug representation with the plain theme and a final newline.
    fn write_debug_plain<WriteT>(&self, writer: &mut WriteT) -> Result<()>
    where
        WriteT: Write,
    {
        let theme = Theme::plain();
        self.write_debug_representation(writer, &DebugPrefix::new(theme.delimiter), &theme)?;
        writeln!(writer)
    }

    /// Print the debug representation to [anstream::stdout] with the default theme and a final newline.
    ///
    /// Panics on write [Error].
    fn print_debug(&self) {
        self.write_debug(&mut anstream::stdout()).unwrap();
    }

    /// Print the debug representation to [stdout] with the plain theme and a final newline.
    ///
    /// Panics on write [Error].
    fn print_debug_plain(&self) {
        self.write_debug_plain(&mut stdout()).unwrap();
    }

    /// Print the debug representation to [anstream::stderr] with the default theme and a final newline.
    ///
    /// Panics on write [Error].
    fn eprint_debug(&self) {
        self.write_debug(&mut anstream::stdout()).unwrap();
    }

    /// Print the debug representation to [stderr] with the plain theme and a final newline.
    ///
    /// Panics on write [Error].
    fn eprint_debug_plain(&self) {
        self.write_debug_plain(&mut stdout()).unwrap();
    }

    /// Capture [write_debug_representation](Debuggable::write_debug_representation) into a string.
    fn to_debug_string(&self, theme: &Theme) -> Result<String> {
        let mut writer = BufWriter::new(Vec::new());
        self.write_debug_representation(&mut writer, &DebugPrefix::new(theme.delimiter), theme)?;
        match String::from_utf8(writer.into_inner().unwrap().into()) {
            Ok(string) => Ok(string),
            Err(error) => Err(Error::new(ErrorKind::Other, format!("{}", error))),
        }
    }
}
