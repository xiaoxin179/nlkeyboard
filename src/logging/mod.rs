pub mod retention;

use std::{fs, panic};

use anyhow::Context;
use time::{UtcOffset, macros::format_description};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, time::OffsetTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

use crate::platform::AppPaths;

pub fn init(paths: &AppPaths) -> anyhow::Result<WorkerGuard> {
    fs::create_dir_all(&paths.log_dir)
        .with_context(|| format!("failed to create log directory {}", paths.log_dir.display()))?;

    let file_appender = tracing_appender::rolling::daily(&paths.log_dir, "voiceinput.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let timer = OffsetTime::new(
        UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    );
    let filter =
        std::env::var("NLKEYBOARD_LOG").unwrap_or_else(|_| "info,nlkeyboard=debug".to_string());
    let env_filter = EnvFilter::try_new(filter).context("invalid NLKEYBOARD_LOG filter")?;

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().with_timer(timer).with_writer(non_blocking))
        .with(fmt::layer().with_writer(std::io::stderr))
        .try_init()
        .context("failed to install tracing subscriber")?;

    panic::set_hook(Box::new(|panic_info| {
        tracing::error!(panic = %panic_info, "application panicked");
    }));

    Ok(guard)
}
