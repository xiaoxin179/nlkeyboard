use crate::{config::AppConfig, platform::AppPaths};

pub fn show(paths: &AppPaths, config: &AppConfig) -> anyhow::Result<()> {
    show_message_box(&build_message(paths, config))
}

fn build_message(paths: &AppPaths, config: &AppConfig) -> String {
    format!(
        "nlkeyboard 已启动\n\n当前版本：启动骨架 + 配置/日志验证\n默认热键：{}\n\n配置文件：\n{}\n\n日志目录：\n{}\n\n下一步会接入托盘、设置页和悬浮窗。",
        config.app.hotkey,
        paths.config_file.display(),
        paths.log_dir.display(),
    )
}

#[cfg(windows)]
fn show_message_box(message: &str) -> anyhow::Result<()> {
    use windows::{
        Win32::UI::WindowsAndMessaging::{MB_ICONINFORMATION, MB_OK, MessageBoxW},
        core::w,
    };

    let wide_message: Vec<u16> = message.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        MessageBoxW(
            None,
            windows::core::PCWSTR(wide_message.as_ptr()),
            w!("nlkeyboard"),
            MB_OK | MB_ICONINFORMATION,
        );
    }

    Ok(())
}

#[cfg(not(windows))]
fn show_message_box(message: &str) -> anyhow::Result<()> {
    println!("{message}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::build_message;
    use crate::{config::AppConfig, platform::AppPaths};

    #[test]
    fn startup_message_contains_user_visible_paths() {
        let paths = AppPaths {
            exe_path: PathBuf::from("nlkeyboard.exe"),
            config_dir: PathBuf::from(r"C:\Users\demo\AppData\Roaming\VoiceInput"),
            config_file: PathBuf::from(r"C:\Users\demo\AppData\Roaming\VoiceInput\config.toml"),
            log_dir: PathBuf::from(r"C:\Users\demo\AppData\Local\VoiceInput\logs"),
            model_dir: PathBuf::from(r"C:\Program Files\VoiceInput\models\sensevoice-small-int8"),
        };

        let message = build_message(&paths, &AppConfig::default());

        assert!(message.contains("nlkeyboard 已启动"));
        assert!(message.contains("Ctrl+Alt+Space"));
        assert!(message.contains("config.toml"));
        assert!(message.contains("logs"));
    }
}
