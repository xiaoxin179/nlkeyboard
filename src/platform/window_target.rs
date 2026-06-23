#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasteOutcome {
    Pasted,
    CopiedOnly,
}

pub trait WindowTarget {
    fn paste_text_at_cursor(&self, text: &str) -> anyhow::Result<PasteOutcome>;
}

#[derive(Debug, Default)]
pub struct NoopWindowTarget;

impl WindowTarget for NoopWindowTarget {
    fn paste_text_at_cursor(&self, _text: &str) -> anyhow::Result<PasteOutcome> {
        Ok(PasteOutcome::CopiedOnly)
    }
}
