use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsrModelConfig {
    pub model_dir: PathBuf,
    pub language: String,
}
