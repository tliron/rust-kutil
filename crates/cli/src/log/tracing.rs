use anstream::stderr;

/// Initialize a tracing subscriber for stderr.
///
/// * 0: no-op.
/// * 1: [ERROR](tracing::Level::ERROR)
/// * 2: [WARN](tracing::Level::WARN)
/// * 3: [INFO](tracing::Level::INFO)
/// * 4: [DEBUG](tracing::Level::DEBUG)
/// * >=5: [TRACE](tracing::Level::TRACE)
pub fn initialize_tracing(verbosity: u8) {
    if verbosity == 0 {
        return;
    }

    let level = match verbosity {
        1 => tracing::Level::ERROR,
        2 => tracing::Level::WARN,
        3 => tracing::Level::INFO,
        4 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };

    tracing_subscriber::fmt().with_writer(stderr).with_max_level(level).init();
}
