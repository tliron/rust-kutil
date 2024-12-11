use {
    owo_colors::*,
    std::{fmt, io::*},
};

const BRANCH_NEST_LAST: &str = "  ";
const BRANCH_NEST_ONGOING: &str = "│ ";
const BRANCH_NEST_ONGOING_THICK: &str = "┃ ";
const BRANCH_NEST_ONGOING_DOUBLE: &str = "║ ";
const BRANCH_INTO_LAST: &str = "└─";
const BRANCH_INTO_LAST_THICK: &str = "┗━";
const BRANCH_INTO_LAST_DOUBLE: &str = "╚═";
const BRANCH_INTO_ONGOING: &str = "├─";
const BRANCH_INTO_ONGOING_THICK: &str = "┣━";
const BRANCH_INTO_ONGOING_DOUBLE: &str = "╠═";

//
// DebugPrefix
//

/// Debug prefix.
///
/// Utility for implementing multi-line [Debuggable](super::debuggable::Debuggable) output
/// in a nested horizontal tree-like structure.
#[derive(Clone, Default, Debug)]
pub struct DebugPrefix {
    prefix: String,
    style: Style,
}

impl DebugPrefix {
    /// Constructor.
    pub fn new(style: Style) -> Self {
        Self { prefix: String::new(), style }
    }

    /// Clone and add a custom suffix.
    pub fn with(&self, suffix: &str) -> Self {
        let mut prefix = self.prefix.clone();
        prefix += suffix;
        Self { prefix, style: self.style }
    }

    /// Clone and add a branch-style suffix.
    pub fn with_branch(&self, last: bool) -> Self {
        if last {
            self.with(BRANCH_NEST_LAST)
        } else {
            self.with(BRANCH_NEST_ONGOING)
        }
    }

    /// Clone and add a thick branch-style suffix.
    pub fn with_thick_branch(&self, last: bool) -> Self {
        if last {
            self.with(BRANCH_NEST_LAST)
        } else {
            self.with(BRANCH_NEST_ONGOING_THICK)
        }
    }

    /// Clone and add a double branch-style suffix.
    pub fn with_double_branch(&self, last: bool) -> Self {
        if last {
            self.with(BRANCH_NEST_LAST)
        } else {
            self.with(BRANCH_NEST_ONGOING_DOUBLE)
        }
    }

    /// Write a newline plus this prefix.
    pub fn write<WriteT>(&self, writer: &mut WriteT) -> Result<()>
    where
        WriteT: Write,
    {
        write!(writer, "\n{}", self.prefix.style(self.style))
    }

    /// If first is false, write a newline plus this prefix.
    pub fn conditional_write<WriteT>(&self, writer: &mut WriteT, first: bool) -> Result<()>
    where
        WriteT: Write,
    {
        if first {
            Ok(())
        } else {
            self.write(writer)
        }
    }

    /// Write a newline plus this prefix. Then write a custom suffix.
    pub fn write_with<WriteT>(&self, writer: &mut WriteT, suffix: &str) -> Result<()>
    where
        WriteT: Write,
    {
        write!(writer, "\n{}", format!("{}{}", self.prefix, suffix).style(self.style))
    }

    /// If first is false, write a newline plus this prefix. Then write a custom suffix.
    /// The custom suffix is *always* written.
    pub fn conditional_write_with<WriteT>(&self, writer: &mut WriteT, suffix: &str, first: bool) -> Result<()>
    where
        WriteT: Write,
    {
        if first {
            write!(writer, "{}", suffix.style(self.style))
        } else {
            self.write_with(writer, suffix)
        }
    }

    /// Write a newline plus this prefix plus a branch-style suffix.
    pub fn write_with_branch<WriteT>(&self, writer: &mut WriteT, last: bool) -> Result<()>
    where
        WriteT: Write,
    {
        if last {
            self.write_with(writer, BRANCH_INTO_LAST)
        } else {
            self.write_with(writer, BRANCH_INTO_ONGOING)
        }
    }

    /// Write a newline plus this prefix plus a thick branch-style suffix.
    pub fn write_with_thick_branch<WriteT>(&self, writer: &mut WriteT, last: bool) -> Result<()>
    where
        WriteT: Write,
    {
        if last {
            self.write_with(writer, BRANCH_INTO_LAST_THICK)
        } else {
            self.write_with(writer, BRANCH_INTO_ONGOING_THICK)
        }
    }

    /// Write a newline plus this prefix plus a double branch-style suffix.
    pub fn write_with_double_branch<WriteT>(&self, writer: &mut WriteT, last: bool) -> Result<()>
    where
        WriteT: Write,
    {
        if last {
            self.write_with(writer, BRANCH_INTO_LAST_DOUBLE)
        } else {
            self.write_with(writer, BRANCH_INTO_ONGOING_DOUBLE)
        }
    }
}

impl fmt::Display for DebugPrefix {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.prefix, formatter)
    }
}
