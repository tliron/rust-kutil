use {
    anstream::ColorChoice,
    anstyle::*,
    clap::{builder::*, *},
};

pub fn clap_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Cyan.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Blue.on_default())
}

//
// Colorize
//

#[derive(ValueEnum, Clone, Default)]
pub enum Colorize {
    #[default]
    True,
    False,
    Force,
}

impl Colorize {
    pub fn apply(&self) {
        match self {
            Colorize::True => {}
            Colorize::False => ColorChoice::Never.write_global(),
            Colorize::Force => ColorChoice::Always.write_global(),
        }
    }
}
