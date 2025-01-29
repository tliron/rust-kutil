use {clap::*, clap_complete_command::*, std::io};

//
// Completion
//

/// Clap command to generate shell auto-completion scripts.
#[derive(Args)]
pub struct Completion {
    /// shell
    #[arg(value_enum)]
    shell: Shell,
}

impl Completion {
    /// Run command.
    pub fn run<ParserT>(&self)
    where
        ParserT: Parser,
    {
        self.shell.generate(&mut ParserT::command(), &mut io::stdout());
    }
}
