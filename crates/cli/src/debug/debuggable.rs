use super::{context::*, format::*, theme::*};

use std::io::*;

const TO_STRING_BUFFER_CAPACITY: usize = 1024;

//
// Debuggable
//

/// Can write a debug representation of itself that can be indented, styled, and multiline.
///
/// Designed to support complex nested hierarchies.
pub trait Debuggable {
    /// Write the debug representation for the context.
    ///
    /// Required behavior for implementations:
    ///
    /// 1. Representations *must not* end in a newline.
    /// 2. If *not* starting with a newline and *not* empty, *must* call [DebugContext::separate] first.
    /// 3. All lines *after* the first (but *not* the first) *must* start with the [DebugContext] indentation.
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> Result<()>
    where
        WriteT: Write;

    // write_debug

    /// Write the debug representation with the default theme and a final newline.
    fn write_debug_with_format<WriteT>(&self, writer: &mut WriteT, format: DebugFormat) -> Result<()>
    where
        WriteT: Write,
    {
        self.write_debug_for(writer, &DebugContext::new(&Theme::default()).with_format(format))?;
        writeln!(writer)
    }

    /// Write the debug representation with the default theme and a final newline.
    fn write_debug<WriteT>(&self, writer: &mut WriteT) -> Result<()>
    where
        WriteT: Write,
    {
        self.write_debug_with_format(writer, DebugFormat::default())
    }

    /// Write the debug representation with the plain theme and a final newline.
    fn write_debug_plain_with_format<WriteT>(&self, writer: &mut WriteT, format: DebugFormat) -> Result<()>
    where
        WriteT: Write,
    {
        self.write_debug_for(writer, &DebugContext::new(&Theme::plain()).with_format(format))?;
        writeln!(writer)
    }

    /// Write the debug representation with the plain theme and a final newline.
    fn write_debug_plain<WriteT>(&self, writer: &mut WriteT) -> Result<()>
    where
        WriteT: Write,
    {
        self.write_debug_with_format(writer, DebugFormat::default())
    }

    // print_debug

    /// Print the debug representation to [anstream::stdout] with the default theme and a final newline.
    ///
    /// Panics on write [Error].
    fn print_debug_with_format(&self, format: DebugFormat) {
        self.write_debug_with_format(&mut anstream::stdout(), format).expect("write_debug_with_format");
    }

    /// Print the debug representation to [anstream::stdout] with the default theme and a final newline.
    ///
    /// Panics on write [Error].
    fn print_debug(&self) {
        self.print_debug_with_format(DebugFormat::default());
    }

    /// Print the debug representation to [stdout] with the plain theme and a final newline.
    ///
    /// Panics on write [Error].
    fn print_debug_plain_with_format(&self, format: DebugFormat) {
        self.write_debug_plain_with_format(&mut stdout(), format).expect("write_debug_plain_with_format");
    }

    /// Print the debug representation to [stdout] with the plain theme and a final newline.
    ///
    /// Panics on write [Error].
    fn print_debug_plain(&self) {
        self.print_debug_plain_with_format(DebugFormat::default());
    }

    // eprint_debug

    /// Print the debug representation to [anstream::stderr] with the default theme and a final newline.
    ///
    /// Panics on write [Error].
    fn eprint_debug_with_format(&self, format: DebugFormat) {
        self.write_debug_with_format(&mut anstream::stdout(), format).expect("write_debug_with_format");
    }

    /// Print the debug representation to [anstream::stderr] with the default theme and a final newline.
    ///
    /// Panics on write [Error].
    fn eprint_debug(&self) {
        self.eprint_debug_with_format(DebugFormat::default());
    }

    /// Print the debug representation to [stderr] with the plain theme and a final newline.
    ///
    /// Panics on write [Error].
    fn eprint_debug_plain_with_format(&self, format: DebugFormat) {
        self.write_debug_plain_with_format(&mut stdout(), format).expect("write_debug_plain_with_format");
    }

    /// Print the debug representation to [stderr] with the plain theme and a final newline.
    ///
    /// Panics on write [Error].
    fn eprint_debug_plain(&self) {
        self.eprint_debug_plain_with_format(DebugFormat::default());
    }

    /// Capture [write_debug_for](Debuggable::write_debug_for) into a string.
    fn to_debug_string_with_format(&self, theme: &Theme, format: DebugFormat) -> Result<String> {
        let mut writer = Vec::with_capacity(TO_STRING_BUFFER_CAPACITY);
        self.write_debug_for(&mut writer, &DebugContext::new(theme).with_format(format))?;
        String::from_utf8(writer.into()).map_err(Error::other)
    }

    /// Capture [write_debug_for](Debuggable::write_debug_for) into a string.
    fn to_debug_string(&self, theme: &Theme) -> Result<String> {
        self.to_debug_string_with_format(theme, DebugFormat::default())
    }
}
