use {clap::*, clap_complete_command::*, std::io};

//
// Completion
//

/// Clap command to generate shell autocompletion scripts.
#[derive(Args)]
pub struct Completion {
    /// shell
    #[arg(value_enum)]
    shell: Shell,
}

impl Completion {
    /// Run command.
    pub fn run<C: Parser>(&self) {
        self.shell.generate(&mut C::command(), &mut io::stdout());
    }
}
