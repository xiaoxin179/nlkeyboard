#[derive(Debug, Clone, PartialEq)]
pub struct AudioSegment {
    pub segment_id: u64,
    pub sample_rate: u32,
    pub samples: Vec<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmenterConfig {
    pub window_ms: u64,
}

impl Default for SegmenterConfig {
    fn default() -> Self {
        Self { window_ms: 1500 }
    }
}
