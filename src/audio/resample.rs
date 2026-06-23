#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioFormat {
    pub sample_rate: u32,
    pub channels: u16,
}

pub const ASR_AUDIO_FORMAT: AudioFormat = AudioFormat {
    sample_rate: 16_000,
    channels: 1,
};
