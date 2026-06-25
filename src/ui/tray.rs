#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayAction {
    StartOrStop,
    OpenSettings,
    OpenLogs,
    Exit,
}

use crate::{config::AppConfig, platform::AppPaths};

pub struct TrayApplication;

impl TrayApplication {
    pub fn run(paths: &AppPaths, config: &AppConfig) -> anyhow::Result<()> {
        run_tray_app(paths, config)
    }
}

#[cfg(windows)]
fn run_tray_app(paths: &AppPaths, config: &AppConfig) -> anyhow::Result<()> {
    windows_tray::run(paths, config)
}

#[cfg(not(windows))]
fn run_tray_app(_paths: &AppPaths, _config: &AppConfig) -> anyhow::Result<()> {
    anyhow::bail!("tray application is only supported on Windows")
}

#[cfg(windows)]
mod windows_tray {
    use std::{mem::size_of, sync::OnceLock};

    use anyhow::Context;
    use windows::{
        Win32::{
            Foundation::{GetLastError, HINSTANCE, HWND, LPARAM, LRESULT, POINT, WPARAM},
            System::LibraryLoader::GetModuleHandleW,
            UI::{
                Shell::{
                    NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE, NIM_SETVERSION,
                    NOTIFYICON_VERSION_4, NOTIFYICONDATAW, Shell_NotifyIconW,
                },
                WindowsAndMessaging::{
                    AppendMenuW, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, CreatePopupMenu,
                    CreateWindowExW, DefWindowProcW, DestroyIcon, DestroyMenu, DispatchMessageW,
                    GetCursorPos, GetMessageW, HICON, IDI_APPLICATION, LoadIconW,
                    MENU_ITEM_FLAGS, MSG, PostQuitMessage, RegisterClassW, SetForegroundWindow,
                    TPM_RIGHTBUTTON, TrackPopupMenu, TranslateMessage, WM_APP, WM_COMMAND,
                    WM_DESTROY, WM_RBUTTONUP, WNDCLASSW, WS_OVERLAPPEDWINDOW,
                },
            },
        },
        core::w,
    };

    use crate::{config::AppConfig, platform::AppPaths};

    const TRAY_UID: u32 = 1;
    const WM_TRAYICON: u32 = WM_APP + 1;
    const MF_DISABLED: MENU_ITEM_FLAGS = MENU_ITEM_FLAGS(0x0002);
    const MENU_OPEN_LOGS: usize = 1001;
    const MENU_EXIT: usize = 1002;

    static LOG_DIR: OnceLock<String> = OnceLock::new();

    pub fn run(paths: &AppPaths, config: &AppConfig) -> anyhow::Result<()> {
        let _ = LOG_DIR.set(paths.log_dir.display().to_string());

        let class_name = w!("nlkeyboard_tray_window");
        let window_title = w!("nlkeyboard");

        let module = unsafe { GetModuleHandleW(None).context("failed to get module handle")? };
        let instance = HINSTANCE(module.0);
        let window_class = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: instance,
            lpszClassName: class_name,
            style: CS_HREDRAW | CS_VREDRAW,
            ..Default::default()
        };

        let atom = unsafe { RegisterClassW(&window_class) };
        if atom == 0 {
            anyhow::bail!("failed to register tray window class: {:?}", unsafe {
                GetLastError()
            });
        }

        let hwnd = unsafe {
            CreateWindowExW(
                Default::default(),
                class_name,
                window_title,
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                Some(instance),
                None,
            )
        }
        .context("failed to create tray window")?;

        let icon = load_tray_icon().context("failed to load tray icon")?;
        add_tray_icon(hwnd, icon, config)?;
        tracing::info!("tray icon added");

        let mut message = MSG::default();
        while unsafe { GetMessageW(&mut message, None, 0, 0) }.into() {
            unsafe {
                let _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }

        remove_tray_icon(hwnd);
        if !icon.0.is_null() {
            unsafe {
                let _ = DestroyIcon(icon);
            }
        }

        tracing::info!("tray application exited");
        Ok(())
    }

    fn load_tray_icon() -> anyhow::Result<HICON> {
        unsafe { LoadIconW(None, IDI_APPLICATION) }.context("LoadIconW failed")
    }

    fn add_tray_icon(hwnd: HWND, icon: HICON, config: &AppConfig) -> anyhow::Result<()> {
        let tip = format!("nlkeyboard\n热键：{}", config.app.hotkey);
        let mut data = NOTIFYICONDATAW {
            cbSize: size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: TRAY_UID,
            uFlags: NIF_MESSAGE | NIF_ICON | NIF_TIP,
            uCallbackMessage: WM_TRAYICON,
            hIcon: icon,
            ..Default::default()
        };
        copy_wide_to_fixed(&tip, &mut data.szTip);

        let added = unsafe { Shell_NotifyIconW(NIM_ADD, &data) };
        if !added.as_bool() {
            anyhow::bail!("Shell_NotifyIconW(NIM_ADD) failed: {:?}", unsafe {
                GetLastError()
            });
        }

        data.Anonymous.uVersion = NOTIFYICON_VERSION_4;
        unsafe {
            let _ = Shell_NotifyIconW(NIM_SETVERSION, &data);
        }

        Ok(())
    }

    fn remove_tray_icon(hwnd: HWND) {
        let data = NOTIFYICONDATAW {
            cbSize: size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: TRAY_UID,
            ..Default::default()
        };

        unsafe {
            let _ = Shell_NotifyIconW(NIM_DELETE, &data);
        }
    }

    extern "system" fn window_proc(
        hwnd: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match message {
            WM_TRAYICON => {
                if lparam.0 as u32 == WM_RBUTTONUP {
                    show_context_menu(hwnd);
                }
                LRESULT(0)
            }
            WM_COMMAND => {
                let command_id = loword(wparam.0 as u32) as usize;
                match command_id {
                    MENU_OPEN_LOGS => open_logs_dir(),
                    MENU_EXIT => unsafe { PostQuitMessage(0) },
                    _ => {}
                }
                LRESULT(0)
            }
            WM_DESTROY => {
                unsafe { PostQuitMessage(0) };
                LRESULT(0)
            }
            _ => unsafe { DefWindowProcW(hwnd, message, wparam, lparam) },
        }
    }

    fn show_context_menu(hwnd: HWND) {
        unsafe {
            let Ok(menu) = CreatePopupMenu() else {
                return;
            };
            if menu.0.is_null() {
                return;
            }

            let _ = AppendMenuW(menu, MF_DISABLED, 0, w!("nlkeyboard 正在运行"));
            let _ = AppendMenuW(menu, MENU_ITEM_FLAGS(0), MENU_OPEN_LOGS, w!("打开日志目录"));
            let _ = AppendMenuW(menu, MENU_ITEM_FLAGS(0), MENU_EXIT, w!("退出"));

            let mut point = POINT::default();
            let _ = GetCursorPos(&mut point);
            let _ = SetForegroundWindow(hwnd);
            let _ = TrackPopupMenu(menu, TPM_RIGHTBUTTON, point.x, point.y, Some(0), hwnd, None);
            let _ = DestroyMenu(menu);
        }
    }

    fn open_logs_dir() {
        let log_dir = LOG_DIR.get().cloned();
        let Some(log_dir) = log_dir else {
            return;
        };

        if let Err(error) = std::process::Command::new("explorer").arg(log_dir).spawn() {
            tracing::error!(%error, "failed to open log directory");
        }
    }

    fn copy_wide_to_fixed(value: &str, output: &mut [u16]) {
        let max_len = output.len().saturating_sub(1);
        for (index, code_unit) in value.encode_utf16().take(max_len).enumerate() {
            output[index] = code_unit;
        }
        output[max_len] = 0;
    }

    fn loword(value: u32) -> u16 {
        (value & 0xffff) as u16
    }
}
