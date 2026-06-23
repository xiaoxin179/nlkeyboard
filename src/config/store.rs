use std::{fs, path::PathBuf};

use anyhow::Context;

use crate::config::schema::AppConfig;

#[derive(Debug, Clone)]
pub struct ConfigStore {
    path: PathBuf,
}

impl ConfigStore {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn load_or_create_default(&self) -> anyhow::Result<AppConfig> {
        if self.path.exists() {
            return self.load();
        }

        let config = AppConfig::default();
        self.save(&config)?;
        Ok(config)
    }

    pub fn load(&self) -> anyhow::Result<AppConfig> {
        let content = fs::read_to_string(&self.path)
            .with_context(|| format!("failed to read config {}", self.path.display()))?;
        toml::from_str(&content)
            .with_context(|| format!("failed to parse config {}", self.path.display()))
    }

    pub fn save(&self, config: &AppConfig) -> anyhow::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create config directory {}", parent.display())
            })?;
        }

        let content = toml::to_string_pretty(config).context("failed to serialize config")?;
        fs::write(&self.path, content)
            .with_context(|| format!("failed to write config {}", self.path.display()))
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;
    use crate::config::schema::FloatingPosition;

    #[test]
    fn creates_default_config_when_missing() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("VoiceInput").join("config.toml");
        let store = ConfigStore::new(path.clone());

        let config = store.load_or_create_default().expect("config");

        assert_eq!(config.app.hotkey, "Ctrl+Alt+Space");
        assert!(!config.app.autostart);
        assert!(config.app.start_minimized);
        assert_eq!(config.ui.floating_position, FloatingPosition::NearCursor);
        assert!(path.exists());
    }
}
