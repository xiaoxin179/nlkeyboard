use anyhow::Context;
use tracing::{info, warn};

use crate::{
    app::{commands::AppCommand, state::AppState},
    config::{AppConfig, ConfigStore},
    platform::{AppPaths, Autostart},
};

pub struct AppSession {
    state: AppState,
    paths: AppPaths,
    config_store: ConfigStore,
    config: AppConfig,
}

impl AppSession {
    pub fn new(paths: AppPaths, config_store: ConfigStore, config: AppConfig) -> Self {
        Self {
            state: AppState::Idle,
            paths,
            config_store,
            config,
        }
    }

    pub fn handle_command(&mut self, command: AppCommand) -> anyhow::Result<()> {
        match command {
            AppCommand::Start => self.start_recording(),
            AppCommand::Stop => self.stop_recording(),
            AppCommand::OpenSettings => {
                info!("settings requested");
                Ok(())
            }
            AppCommand::SaveSettings => self.save_settings(),
            AppCommand::Exit => {
                info!("exit requested");
                Ok(())
            }
        }
    }

    fn start_recording(&mut self) -> anyhow::Result<()> {
        if self.state != AppState::Idle {
            warn!(state = ?self.state, "ignored start command outside idle state");
            return Ok(());
        }

        self.state = AppState::Recording;
        info!(hotkey = %self.config.app.hotkey, "recording started");
        Ok(())
    }

    fn stop_recording(&mut self) -> anyhow::Result<()> {
        if self.state != AppState::Recording {
            warn!(state = ?self.state, "ignored stop command outside recording state");
            return Ok(());
        }

        self.state = AppState::Stopping;
        info!("recording stopping");

        self.state = AppState::Idle;
        info!("recording stopped");
        Ok(())
    }

    fn save_settings(&self) -> anyhow::Result<()> {
        self.config_store
            .save(&self.config)
            .context("failed to save configuration")?;

        let autostart = Autostart::new("nlkeyboard");
        autostart
            .set_enabled(self.config.app.autostart, &self.paths.exe_path)
            .context("failed to update autostart setting")?;

        Ok(())
    }
}
