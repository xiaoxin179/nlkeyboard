use std::path::Path;

#[derive(Debug, Clone)]
pub struct Autostart {
    app_name: String,
}

impl Autostart {
    pub fn new(app_name: impl Into<String>) -> Self {
        Self {
            app_name: app_name.into(),
        }
    }

    pub fn set_enabled(&self, enabled: bool, exe_path: &Path) -> anyhow::Result<()> {
        set_autostart(&self.app_name, enabled, exe_path)
    }
}

#[cfg(windows)]
fn set_autostart(app_name: &str, enabled: bool, exe_path: &Path) -> anyhow::Result<()> {
    use anyhow::Context;
    use winreg::{RegKey, enums::HKEY_CURRENT_USER};

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (run_key, _) = hkcu
        .create_subkey(r"Software\Microsoft\Windows\CurrentVersion\Run")
        .context("failed to open current user Run registry key")?;

    if enabled {
        let command = format!("\"{}\" --minimized", exe_path.display());
        run_key
            .set_value(app_name, &command)
            .context("failed to set autostart registry value")?;
    } else {
        match run_key.delete_value(app_name) {
            Ok(()) => {}
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => return Err(err).context("failed to delete autostart registry value"),
        }
    }

    Ok(())
}

#[cfg(not(windows))]
fn set_autostart(_app_name: &str, _enabled: bool, _exe_path: &Path) -> anyhow::Result<()> {
    anyhow::bail!("autostart is only supported on Windows")
}
