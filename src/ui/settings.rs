use crate::config::AppConfig;

#[derive(Debug, Clone, PartialEq)]
pub struct SettingsViewModel {
    pub config: AppConfig,
    pub ai_connection_status: Option<String>,
    pub hotkey_error: Option<String>,
}

impl SettingsViewModel {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            ai_connection_status: None,
            hotkey_error: None,
        }
    }
}
