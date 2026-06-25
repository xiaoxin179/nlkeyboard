pub mod commands;
pub mod session;
pub mod state;

use anyhow::Context;
use tracing::info;

use crate::{
    config::{AppConfig, ConfigStore},
    logging,
    platform::AppPaths,
    ui,
};

pub fn run() -> anyhow::Result<()> {
    let paths = AppPaths::discover().context("failed to discover application paths")?;
    let _log_guard = logging::init(&paths).context("failed to initialize logging")?;

    info!(version = env!("CARGO_PKG_VERSION"), "starting nlkeyboard");
    info!(config_path = %paths.config_file.display(), "using config file");
    info!(log_dir = %paths.log_dir.display(), "using log directory");

    let config_store = ConfigStore::new(paths.config_file.clone());
    let config = config_store.load_or_create_default()?;
    info!(hotkey = %config.app.hotkey, "configuration loaded");

    let _session = session::AppSession::new(paths.clone(), config_store, config.clone());
    info!("starting tray application");
    ui::tray::TrayApplication::run(&paths, &config).context("failed to run tray application")?;

    info!("nlkeyboard bootstrap finished");
    Ok(())
}

pub fn default_config() -> AppConfig {
    AppConfig::default()
}
