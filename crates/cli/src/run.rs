use super::exit::*;

use {
    anstream::eprintln,
    owo_colors::OwoColorize,
    std::{fmt, process::*},
};

pub type Runner<E> = fn() -> Result<(), E>;

/// Runs a [Runner], handling a returned [Exit].
pub fn run<E: HasExit + fmt::Display>(run: Runner<E>) -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,

        Err(err) => match err.get_exit() {
            Some(exit) => {
                if let Some(message) = &exit.message {
                    eprintln!("{}", message.red());
                }
                ExitCode::from(exit.code)
            }

            _ => {
                eprintln!("{}", err.red());
                ExitCode::FAILURE
            }
        },
    }
}
