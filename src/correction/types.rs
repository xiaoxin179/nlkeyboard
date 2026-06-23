use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CorrectionRequest {
    pub previous_sentence: String,
    pub current_sentence: String,
    pub full_draft: String,
    pub locale: String,
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CorrectionResponse {
    pub corrected_previous_sentence: String,
    pub corrected_current_sentence: String,
}
