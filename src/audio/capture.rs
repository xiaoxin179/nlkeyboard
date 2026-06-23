#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioDeviceInfo {
    pub name: String,
    pub sample_rate: u32,
    pub channels: u16,
}

pub trait AudioCapture {
    fn default_device(&self) -> anyhow::Result<AudioDeviceInfo>;
}
