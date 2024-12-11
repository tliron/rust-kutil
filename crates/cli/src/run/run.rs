use super::exit::*;

use {
    anstream::eprintln,
    owo_colors::OwoColorize,
    std::{fmt, process::*},
};

//
// Runner
//

/// A replacement for `main`.
pub type Runner<ErrorT> = fn() -> Result<(), ErrorT>;

/// Runs a [Runner], handling a returned [Exit].
///
/// If the exit has a goodbye message, it will be printed to stderr. If
/// the exit code is an error (non-zero) it will be in red.
///
/// Non-exit errors will be displayed in red.
///
/// Designed to be the only content for your `main` function. The
/// good stuff should go into your [Runner].
pub fn run<ErrorT>(run: Runner<ErrorT>) -> ExitCode
where
    ErrorT: HasExit + fmt::Display,
{
    match run() {
        Ok(_) => ExitCode::SUCCESS,

        Err(error) => match error.get_exit() {
            Some(exit) => {
                if let Some(message) = &exit.message {
                    if exit.code == 0 {
                        eprintln!("{}", message);
                    } else {
                        eprintln!("{}", message.red());
                    }
                }
                ExitCode::from(exit.code)
            }

            _ => {
                eprintln!("{}", error.red());
                ExitCode::FAILURE
            }
        },
    }
}
