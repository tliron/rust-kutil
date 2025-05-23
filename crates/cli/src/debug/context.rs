use super::{format::*, theme::*};

use std::io::*;

const INDENTATION: &str = "  ";
const BRANCH_CONTINUATION_LAST: &str = "  ";
const BRANCH_CONTINUATION_ONGOING: &str = "│ ";
const BRANCH_CONTINUATION_ONGOING_THICK: &str = "┃ ";
const BRANCH_CONTINUATION_ONGOING_DOUBLE: &str = "║ ";
const BRANCH_INTO_LAST: &str = "└─";
const BRANCH_INTO_LAST_THICK: &str = "┗━";
const BRANCH_INTO_LAST_DOUBLE: &str = "╚═";
const BRANCH_INTO_ONGOING: &str = "├─";
const BRANCH_INTO_ONGOING_THICK: &str = "┣━";
const BRANCH_INTO_ONGOING_DOUBLE: &str = "╠═";

//
// DebugContext
//

/// Debug context.
///
/// Used with [Debuggable](super::debuggable::Debuggable) to keep track of the mode and graphical
/// structure.
#[derive(Clone)]
pub struct DebugContext<'own> {
    /// Theme.
    pub theme: &'own Theme,

    /// Format.
    ///
    /// Defaults to [DebugFormat::Reduced].
    pub format: DebugFormat,

    /// Whether we are continuing a single-line representation.
    ///
    /// Defaults to false.
    pub inline: bool,

    /// Whether we should insert a separator before the representation.
    ///
    /// Defaults to false.
    pub separator: bool,

    /// Indentation for multi-line representations.
    pub indentation: String,
}

impl<'own> DebugContext<'own> {
    /// Constructor.
    pub fn new(theme: &'own Theme) -> Self {
        Self { theme, format: DebugFormat::default(), inline: false, separator: false, indentation: String::new() }
    }

    /// Create child context.
    ///
    /// Will set inline to false and clone the other fields.
    pub fn child(&self) -> Self {
        Self {
            theme: self.theme,
            format: self.format.clone(),
            inline: false,
            separator: self.separator,
            indentation: self.indentation.clone(),
        }
    }

    /// Change the theme.
    pub fn with_theme(mut self, theme: &'own Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Change the format.
    pub fn with_format(mut self, format: DebugFormat) -> Self {
        self.format = format;
        self
    }

    /// Change the inline flag.
    ///
    /// Note that the other "with_" functions set inline to false, so make sure to call
    /// this function at the end of "with_" chains.
    pub fn with_inline(mut self, inline: bool) -> Self {
        self.inline = inline;
        self
    }

    /// Change the separator.
    pub fn with_separator(mut self, separator: bool) -> Self {
        self.separator = separator;
        self
    }

    /// Increase the indentation with spaces.
    pub fn increase_indentation(mut self) -> Self {
        self.indentation = self.indentation + INDENTATION;
        self
    }

    /// Increase the indentation with a branch continuation.
    pub fn increase_indentation_branch(mut self, last: bool) -> Self {
        if last {
            self.indentation = self.indentation + BRANCH_CONTINUATION_LAST;
        } else {
            self.indentation = self.indentation + BRANCH_CONTINUATION_ONGOING;
        }
        self
    }

    /// Increase the indentation with a thick branch continuation.
    pub fn increase_indentation_thick_branch(mut self, last: bool) -> Self {
        if last {
            self.indentation = self.indentation + BRANCH_CONTINUATION_LAST;
        } else {
            self.indentation = self.indentation + BRANCH_CONTINUATION_ONGOING_THICK;
        }
        self
    }

    /// Increase the indentation with a double branch continuation.
    pub fn increase_indentation_double_branch(mut self, last: bool) -> Self {
        if last {
            self.indentation = self.indentation + BRANCH_CONTINUATION_LAST;
        } else {
            self.indentation = self.indentation + BRANCH_CONTINUATION_ONGOING_DOUBLE;
        }
        self
    }

    /// If the separate flag is true, write the separator.
    pub fn separate<WriteT>(&self, writer: &mut WriteT) -> Result<()>
    where
        WriteT: Write,
    {
        match self.separator {
            true => write!(writer, " "),
            false => Ok(()),
        }
    }

    /// Write a newline plus the indentation.
    pub fn indent<WriteT>(&self, writer: &mut WriteT) -> Result<()>
    where
        WriteT: Write,
    {
        write!(writer, "\n{}", self.theme.delimiter(&self.indentation))
    }

    /// Write a newline plus the indentation. Then write a custom delimiter.
    pub fn indent_into<WriteT>(&self, writer: &mut WriteT, delimiter: &str) -> Result<()>
    where
        WriteT: Write,
    {
        write!(writer, "\n{}", self.theme.delimiter(format!("{}{}", self.indentation, delimiter)))
    }

    /// Write a newline plus the indentation plus a branch-into delimiter.
    pub fn indent_into_branch<WriteT>(&self, writer: &mut WriteT, last: bool) -> Result<()>
    where
        WriteT: Write,
    {
        if last {
            self.indent_into(writer, BRANCH_INTO_LAST)
        } else {
            self.indent_into(writer, BRANCH_INTO_ONGOING)
        }
    }

    /// Write a newline plus the indentation plus a thick branch-into delimiter.
    pub fn indent_into_thick_branch<WriteT>(&self, writer: &mut WriteT, last: bool) -> Result<()>
    where
        WriteT: Write,
    {
        if last {
            self.indent_into(writer, BRANCH_INTO_LAST_THICK)
        } else {
            self.indent_into(writer, BRANCH_INTO_ONGOING_THICK)
        }
    }

    /// Write a newline plus the indentation plus a double branch-into delimiter.
    pub fn indent_into_double_branch<WriteT>(&self, writer: &mut WriteT, last: bool) -> Result<()>
    where
        WriteT: Write,
    {
        if last {
            self.indent_into(writer, BRANCH_INTO_LAST_DOUBLE)
        } else {
            self.indent_into(writer, BRANCH_INTO_ONGOING_DOUBLE)
        }
    }

    /// If the inline flag is false and first is true, write the separator. Otherwise write a
    /// newline plus the indentation.
    pub fn separate_or_indent<WriteT>(&self, writer: &mut WriteT, first: bool) -> Result<()>
    where
        WriteT: Write,
    {
        if first && !self.inline {
            self.separate(writer)
        } else {
            self.indent(writer)
        }
    }

    /// If the inline flag is false and first is true write the separator. Otherwise write a
    /// newline plus the indentation. Then write a custom delimiter.
    pub fn separate_or_indent_into<WriteT>(&self, writer: &mut WriteT, delimiter: &str, first: bool) -> Result<()>
    where
        WriteT: Write,
    {
        if first && !self.inline {
            self.separate(writer)?;
            self.theme.write_delimiter(writer, delimiter)
        } else {
            self.indent_into(writer, delimiter)
        }
    }
}
