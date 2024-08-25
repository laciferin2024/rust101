use tracing::{event, span, Level, warn, error, debug, info};
use tracing_subscriber::fmt;

fn main() {

        // Initialize the tracing subscriber to output logs to stdout
    fmt::init();

    debug!("DEBUG");
    info!("INFO");
    warn!("WARN");
    error!("error");
    // records an event outside of any span context:
    event!(Level::INFO, "something happened");

    let span = span!(Level::INFO, "my_span");
    let _guard = span.enter();

    // records an event within "my_span".
    event!(Level::DEBUG, "something happened inside my_span");

    another_function();
}

fn another_function() {
    let function_span = span!(Level::DEBUG, "function_span");
    let _enter = function_span.enter();

    info!("This log message is within 'function_span'");
}