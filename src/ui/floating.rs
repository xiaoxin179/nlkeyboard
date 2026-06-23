#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatingStatus {
    Ready,
    Listening,
    Recognizing,
    Correcting,
    Copied,
    MicrophoneUnavailable,
}

impl FloatingStatus {
    pub fn label(self) -> &'static str {
        match self {
            FloatingStatus::Ready => "准备中",
            FloatingStatus::Listening => "正在听",
            FloatingStatus::Recognizing => "正在识别",
            FloatingStatus::Correcting => "正在校对",
            FloatingStatus::Copied => "已复制",
            FloatingStatus::MicrophoneUnavailable => "麦克风不可用",
        }
    }
}
