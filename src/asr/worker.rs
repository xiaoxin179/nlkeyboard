use crate::asr::types::{AsrRequest, AsrResponse};

pub trait AsrWorker {
    fn recognize(&mut self, request: AsrRequest) -> anyhow::Result<AsrResponse>;
}

#[derive(Debug, Default)]
pub struct StubAsrWorker;

impl AsrWorker for StubAsrWorker {
    fn recognize(&mut self, request: AsrRequest) -> anyhow::Result<AsrResponse> {
        Ok(AsrResponse {
            session_id: request.session_id,
            segment_id: request.segment_id,
            text: String::new(),
            elapsed_ms: 0,
        })
    }
}
