use std::{env, path::PathBuf};

use anyhow::Context;
use directories::BaseDirs;

#[derive(Debug, Clone)]
pub struct AppPaths {
    pub exe_path: PathBuf,
    pub config_dir: PathBuf,
    pub config_file: PathBuf,
    pub log_dir: PathBuf,
    pub model_dir: PathBuf,
}

impl AppPaths {
    pub fn discover() -> anyhow::Result<Self> {
        let base_dirs = BaseDirs::new().context("failed to resolve base directories")?;
        let config_dir = base_dirs.config_dir().join("VoiceInput");
        let config_file = config_dir.join("config.toml");
        let log_dir = base_dirs.data_local_dir().join("VoiceInput").join("logs");
        let model_dir = PathBuf::from(r"C:\Program Files\VoiceInput\models\sensevoice-small-int8");
        let exe_path = env::current_exe().context("failed to resolve current exe path")?;

        Ok(Self {
            exe_path,
            config_dir,
            config_file,
            log_dir,
            model_dir,
        })
    }
}
