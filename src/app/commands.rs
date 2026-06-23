#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppCommand {
    Start,
    Stop,
    OpenSettings,
    SaveSettings,
    Exit,
}
