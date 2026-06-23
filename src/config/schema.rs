use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub app: AppSection,
    pub ui: UiSection,
    pub asr: AsrSection,
    pub correction: CorrectionSection,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app: AppSection::default(),
            ui: UiSection::default(),
            asr: AsrSection::default(),
            correction: CorrectionSection::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppSection {
    pub hotkey: String,
    pub autostart: bool,
    pub start_minimized: bool,
    pub log_retention_days: u32,
    pub log_retention_mb: u64,
}

impl Default for AppSection {
    fn default() -> Self {
        Self {
            hotkey: "Ctrl+Alt+Space".to_string(),
            autostart: false,
            start_minimized: true,
            log_retention_days: 14,
            log_retention_mb: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UiSection {
    pub floating_position: FloatingPosition,
    pub typewriter_chars_per_second: u32,
    pub show_correction_highlight: bool,
}

impl Default for UiSection {
    fn default() -> Self {
        Self {
            floating_position: FloatingPosition::NearCursor,
            typewriter_chars_per_second: 24,
            show_correction_highlight: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FloatingPosition {
    NearCursor,
    BottomCenter,
    LastPosition,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AsrSection {
    pub model_dir: String,
    pub language: String,
}

impl Default for AsrSection {
    fn default() -> Self {
        Self {
            model_dir: r"C:\Program Files\VoiceInput\models\sensevoice-small-int8".to_string(),
            language: "zh-CN".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CorrectionSection {
    pub enabled: bool,
    pub base_url: String,
    pub model: String,
    pub timeout_ms: u64,
    pub style_prompt: String,
}

impl Default for CorrectionSection {
    fn default() -> Self {
        Self {
            enabled: false,
            base_url: String::new(),
            model: String::new(),
            timeout_ms: 3000,
            style_prompt: String::new(),
        }
    }
}
