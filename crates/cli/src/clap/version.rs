use clap::*;

//
// Version
//

/// Clap command to print version.
#[derive(Args, Clone, Debug)]
pub struct Version;

impl Version {
    /// Run command.
    pub fn run<ParserT>(&self)
    where
        ParserT: Parser,
    {
        print!("{}", ParserT::command().render_version());
    }
}
