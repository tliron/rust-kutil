use {
    owo_colors::*,
    std::{fmt, io},
};

//
// Theme
//

/// Collection of theme for printing text.
///
/// See [Debuggable](super::debuggable::Debuggable).
pub struct Theme {
    /// For bare words: true, false, null, etc.
    pub bare_style: Style,

    /// For numbers.
    pub number_style: Style,

    /// For strings and characters.
    pub string_style: Style,

    /// For names of types, instances, etc.
    pub name_style: Style,

    /// For metadata.
    pub meta_style: Style,

    /// For errors.
    pub error_style: Style,

    /// For headings.
    pub heading_style: Style,

    /// For delimiters.
    pub delimiter_style: Style,
}

impl Theme {
    /// Plain theme.
    pub fn plain() -> Self {
        Self {
            bare_style: Style::new(),
            number_style: Style::new(),
            string_style: Style::new(),
            name_style: Style::new(),
            meta_style: Style::new(),
            error_style: Style::new(),
            heading_style: Style::new(),
            delimiter_style: Style::new(),
        }
    }

    /// Apply bare style.
    pub fn bare<ThingT>(&self, thing: ThingT) -> Styled<ThingT> {
        self.bare_style.style(thing)
    }

    /// Apply number style.
    pub fn number<ThingT>(&self, thing: ThingT) -> Styled<ThingT> {
        self.number_style.style(thing)
    }

    /// Apply string style.
    pub fn string<ThingT>(&self, thing: ThingT) -> Styled<ThingT> {
        self.string_style.style(thing)
    }

    /// Apply name style.
    pub fn name<ThingT>(&self, thing: ThingT) -> Styled<ThingT> {
        self.name_style.style(thing)
    }

    /// Apply meta style.
    pub fn meta<ThingT>(&self, thing: ThingT) -> Styled<ThingT> {
        self.meta_style.style(thing)
    }

    /// Apply error style.
    pub fn error<ThingT>(&self, thing: ThingT) -> Styled<ThingT> {
        self.error_style.style(thing)
    }

    /// Apply heading style.
    pub fn heading<ThingT>(&self, thing: ThingT) -> Styled<ThingT> {
        self.heading_style.style(thing)
    }

    /// Apply delimiter style.
    pub fn delimiter<ThingT>(&self, thing: ThingT) -> Styled<ThingT> {
        self.delimiter_style.style(thing)
    }

    /// Write [fmt::Display] in bare style.
    pub fn write_bare<WriteT, ThingT>(&self, writer: &mut WriteT, thing: ThingT) -> io::Result<()>
    where
        WriteT: io::Write,
        ThingT: fmt::Display,
    {
        write!(writer, "{}", self.bare(thing))
    }

    /// Write [fmt::Display] in number style.
    pub fn write_number<WriteT, ThingT>(&self, writer: &mut WriteT, thing: ThingT) -> io::Result<()>
    where
        WriteT: io::Write,
        ThingT: fmt::Display,
    {
        write!(writer, "{}", self.number(thing))
    }

    /// Write [fmt::Display] in string style.
    pub fn write_string<WriteT, ThingT>(&self, writer: &mut WriteT, thing: ThingT) -> io::Result<()>
    where
        WriteT: io::Write,
        ThingT: fmt::Display,
    {
        write!(writer, "{}", self.string(thing))
    }

    /// Write [fmt::Display] in name style.
    pub fn write_name<WriteT, ThingT>(&self, writer: &mut WriteT, thing: ThingT) -> io::Result<()>
    where
        WriteT: io::Write,
        ThingT: fmt::Display,
    {
        write!(writer, "{}", self.name(thing))
    }

    /// Write [fmt::Display] in meta style.
    pub fn write_meta<WriteT, ThingT>(&self, writer: &mut WriteT, thing: ThingT) -> io::Result<()>
    where
        WriteT: io::Write,
        ThingT: fmt::Display,
    {
        write!(writer, "{}", self.meta(thing))
    }

    /// Write [fmt::Display] in error style.
    pub fn write_error<WriteT, ThingT>(&self, writer: &mut WriteT, thing: ThingT) -> io::Result<()>
    where
        WriteT: io::Write,
        ThingT: fmt::Display,
    {
        write!(writer, "{}", self.error(thing))
    }

    /// Write [fmt::Display] in heading style.
    pub fn write_heading<WriteT, ThingT>(&self, writer: &mut WriteT, thing: ThingT) -> io::Result<()>
    where
        WriteT: io::Write,
        ThingT: fmt::Display,
    {
        write!(writer, "{}", self.heading(thing))
    }

    /// Write [fmt::Display] in delimiter style.
    pub fn write_delimiter<WriteT, ThingT>(&self, writer: &mut WriteT, thing: ThingT) -> io::Result<()>
    where
        WriteT: io::Write,
        ThingT: fmt::Display,
    {
        write!(writer, "{}", self.delimiter(thing))
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bare_style: Style::new().yellow(),
            number_style: Style::new().magenta(),
            string_style: Style::new().cyan(),
            name_style: Style::new().green(),
            meta_style: Style::new().blue().italic(),
            error_style: Style::new().red().bold(),
            heading_style: Style::new().green().bold().underline(),
            delimiter_style: Style::new().dimmed(),
        }
    }
}
