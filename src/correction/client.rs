use crate::correction::types::{CorrectionRequest, CorrectionResponse};

pub trait CorrectionClient {
    fn correct(&self, request: CorrectionRequest) -> anyhow::Result<Option<CorrectionResponse>>;
}

#[derive(Debug, Default)]
pub struct DisabledCorrectionClient;

impl CorrectionClient for DisabledCorrectionClient {
    fn correct(&self, _request: CorrectionRequest) -> anyhow::Result<Option<CorrectionResponse>> {
        Ok(None)
    }
}
