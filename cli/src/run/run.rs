use super::run_error::*;

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
    ErrorT: RunError + fmt::Display,
{
    match run() {
        Ok(_) => ExitCode::SUCCESS,

        Err(error) => {
            let (handled, code) = error.handle();

            if !handled {
                let error = error.to_string();
                if !error.is_empty() {
                    eprintln!("{}", error.red());
                }
            }

            ExitCode::from(code)
        }
    }
}
