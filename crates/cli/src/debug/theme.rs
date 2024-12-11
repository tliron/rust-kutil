use owo_colors::*;

//
// Theme
//

/// Collection of theme for printing text.
///
/// See [Debuggable](super::debuggable::Debuggable).
pub struct Theme {
    /// For bare words: true, false, null, etc.
    pub bare: Style,

    /// For numbers.
    pub number: Style,

    /// For strings and characters.
    pub string: Style,

    /// For names of types, instances, etc.
    pub name: Style,

    /// For metadata.
    pub meta: Style,

    /// For errors.
    pub error: Style,

    /// For headings.
    pub heading: Style,

    /// For delimiters.
    pub delimiter: Style,
}

impl Theme {
    /// Plain theme.
    pub fn plain() -> Self {
        Self {
            bare: Style::new(),
            number: Style::new(),
            string: Style::new(),
            name: Style::new(),
            meta: Style::new(),
            error: Style::new(),
            heading: Style::new(),
            delimiter: Style::new(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bare: Style::new().yellow(),
            number: Style::new().magenta(),
            string: Style::new().cyan(),
            name: Style::new().green(),
            meta: Style::new().blue().italic(),
            error: Style::new().red().bold(),
            heading: Style::new().green().bold().underline(),
            delimiter: Style::new().dimmed(),
        }
    }
}
