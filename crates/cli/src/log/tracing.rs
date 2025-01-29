use {
    anstream::stderr,
    std::{fs::*, io::*, path::*},
    time::{format_description::*, macros::format_description},
    tracing_subscriber::{fmt::time::*, *},
};

// RFC 3339 with subseconds
// Or ISO 8601 with fewer subsecond digits
// See: https://time-rs.github.io/book/api/well-known-format-descriptions.html
const TIME_FORMAT: &[BorrowedFormatItem<'_>] = format_description!(
    "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3][offset_hour]:[offset_minute]"
);

/// Initialize a tracing subscriber for stderr.
///
/// * 0: no tracing subscriber.
/// * 1: [ERROR](tracing::Level::ERROR)
/// * 2: [WARN](tracing::Level::WARN)
/// * 3: [INFO](tracing::Level::INFO)
/// * 4: [DEBUG](tracing::Level::DEBUG)
/// * >=5: [TRACE](tracing::Level::TRACE)
pub fn initialize_tracing(verbosity: u8, path: Option<&PathBuf>) -> Result<()> {
    if verbosity == 0 {
        return Ok(());
    }

    let level = match verbosity {
        1 => tracing::Level::ERROR,
        2 => tracing::Level::WARN,
        3 => tracing::Level::INFO,
        4 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };

    let timer = LocalTime::new(TIME_FORMAT);

    let builder = fmt().with_max_level(level).with_timer(timer);

    match path {
        Some(path) => {
            let file = OpenOptions::new().write(true).create(true).append(true).open(path)?;
            builder.with_writer(file).with_ansi(false).init();
        }

        None => builder.with_writer(stderr).init(),
    };

    Ok(())
}
