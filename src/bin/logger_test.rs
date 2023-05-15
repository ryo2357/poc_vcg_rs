use mylogger::{debug, error, info, init, warn};

fn main() {
    init();

    debug!("debug message");
    info!("informational message");
    warn!("warning message");
    error!("this is an error {}", "message");
}
