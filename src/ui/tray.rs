#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayAction {
    StartOrStop,
    OpenSettings,
    OpenLogs,
    Exit,
}
