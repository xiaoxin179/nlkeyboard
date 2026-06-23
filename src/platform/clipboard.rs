#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardSnapshot {
    text: Option<String>,
}

impl ClipboardSnapshot {
    pub fn empty() -> Self {
        Self { text: None }
    }

    pub fn from_text(text: Option<String>) -> Self {
        Self { text }
    }

    pub fn into_text(self) -> Option<String> {
        self.text
    }
}

pub trait Clipboard {
    fn snapshot(&mut self) -> anyhow::Result<ClipboardSnapshot>;
    fn set_text(&mut self, text: &str) -> anyhow::Result<()>;
    fn restore(&mut self, snapshot: ClipboardSnapshot) -> anyhow::Result<()>;
}

#[derive(Debug, Default)]
pub struct MemoryClipboard {
    text: Option<String>,
}

impl Clipboard for MemoryClipboard {
    fn snapshot(&mut self) -> anyhow::Result<ClipboardSnapshot> {
        Ok(ClipboardSnapshot::from_text(self.text.clone()))
    }

    fn set_text(&mut self, text: &str) -> anyhow::Result<()> {
        self.text = Some(text.to_string());
        Ok(())
    }

    fn restore(&mut self, snapshot: ClipboardSnapshot) -> anyhow::Result<()> {
        self.text = snapshot.into_text();
        Ok(())
    }
}
