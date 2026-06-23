use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetentionPolicy {
    pub max_days: u32,
    pub max_total_mb: u64,
}

impl RetentionPolicy {
    pub fn new(max_days: u32, max_total_mb: u64) -> Self {
        Self {
            max_days,
            max_total_mb,
        }
    }
}

pub fn cleanup_logs(_log_dir: &Path, _policy: RetentionPolicy) -> anyhow::Result<()> {
    Ok(())
}
