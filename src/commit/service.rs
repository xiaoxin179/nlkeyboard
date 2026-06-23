use crate::platform::{
    clipboard::Clipboard,
    window_target::{PasteOutcome, WindowTarget},
};

pub struct CommitService<C, W> {
    clipboard: C,
    window_target: W,
}

impl<C, W> CommitService<C, W>
where
    C: Clipboard,
    W: WindowTarget,
{
    pub fn new(clipboard: C, window_target: W) -> Self {
        Self {
            clipboard,
            window_target,
        }
    }

    pub fn commit_text(&mut self, text: &str) -> anyhow::Result<PasteOutcome> {
        let snapshot = self.clipboard.snapshot()?;
        self.clipboard.set_text(text)?;

        let outcome = self.window_target.paste_text_at_cursor(text)?;
        if outcome == PasteOutcome::Pasted {
            self.clipboard.restore(snapshot)?;
        }

        Ok(outcome)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::{
        clipboard::{ClipboardSnapshot, MemoryClipboard},
        window_target::NoopWindowTarget,
    };

    #[test]
    fn keeps_text_on_clipboard_when_target_is_not_editable() {
        let clipboard = MemoryClipboard::default();
        let window_target = NoopWindowTarget;
        let mut service = CommitService::new(clipboard, window_target);

        let outcome = service.commit_text("hello").expect("commit");

        assert_eq!(outcome, PasteOutcome::CopiedOnly);
    }

    struct PastedWindowTarget;

    impl WindowTarget for PastedWindowTarget {
        fn paste_text_at_cursor(&self, _text: &str) -> anyhow::Result<PasteOutcome> {
            Ok(PasteOutcome::Pasted)
        }
    }

    #[derive(Debug, Default)]
    struct InspectableClipboard {
        text: Option<String>,
    }

    impl Clipboard for InspectableClipboard {
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

    #[test]
    fn restores_clipboard_when_paste_succeeds() {
        let clipboard = InspectableClipboard {
            text: Some("old".to_string()),
        };
        let window_target = PastedWindowTarget;
        let mut service = CommitService::new(clipboard, window_target);

        let outcome = service.commit_text("new").expect("commit");

        assert_eq!(outcome, PasteOutcome::Pasted);
        assert_eq!(service.clipboard.text.as_deref(), Some("old"));
    }
}
