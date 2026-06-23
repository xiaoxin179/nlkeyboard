#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HotkeyBinding {
    pub display: String,
}

impl HotkeyBinding {
    pub fn parse(input: impl Into<String>) -> anyhow::Result<Self> {
        let display = input.into().trim().to_string();
        if display.is_empty() {
            anyhow::bail!("hotkey cannot be empty");
        }

        Ok(Self { display })
    }
}

pub trait HotkeyRegistrar {
    fn register(&mut self, binding: &HotkeyBinding) -> anyhow::Result<()>;
    fn unregister(&mut self) -> anyhow::Result<()>;
}

#[derive(Debug, Default)]
pub struct NoopHotkeyRegistrar {
    active: Option<HotkeyBinding>,
}

impl HotkeyRegistrar for NoopHotkeyRegistrar {
    fn register(&mut self, binding: &HotkeyBinding) -> anyhow::Result<()> {
        self.active = Some(binding.clone());
        Ok(())
    }

    fn unregister(&mut self) -> anyhow::Result<()> {
        self.active = None;
        Ok(())
    }
}
