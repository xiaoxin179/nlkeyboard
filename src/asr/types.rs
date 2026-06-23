#[derive(Debug, Clone, PartialEq)]
pub struct AsrRequest {
    pub session_id: String,
    pub segment_id: u64,
    pub sample_rate: u32,
    pub samples: Vec<f32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsrResponse {
    pub session_id: String,
    pub segment_id: u64,
    pub text: String,
    pub elapsed_ms: u64,
}
