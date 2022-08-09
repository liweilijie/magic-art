mod config;

use tracing::{debug, info, trace, warn};
use time::macros::format_description;
use time::UtcOffset;
use tracing_subscriber::fmt::time::OffsetTime;

fn main() {
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "magic_art=debug".to_string()),
        ))
        .with_timer(local_time)
        .init();

    info!("info.");
    warn!("warn.");
    debug!("debug.");
    trace!("trace.");

    let _ = config::read_config("config.toml").expect("failed to read config");
}
