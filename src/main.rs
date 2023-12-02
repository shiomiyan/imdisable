#![windows_subsystem = "windows"]

use device_query::{
    DeviceQuery, DeviceState, Keycode::CapsLock, Keycode::LControl, Keycode::LeftBracket,
};
use std::{thread, time::Duration};

fn main() {
    let device_state = DeviceState::new();
    loop {
        let keys = device_state.get_keys();
        if (keys.contains(&LControl) || keys.contains(&CapsLock)) && keys.contains(&LeftBracket) {
            imdisable().ok();
        }
        thread::sleep(Duration::from_millis(50));
    }
}

#[cfg(windows)]
use windows::Win32::{
    Foundation::{LPARAM, WPARAM},
    UI::{
        Input::Ime::{ImmGetDefaultIMEWnd, IMC_SETOPENSTATUS},
        WindowsAndMessaging::{GetForegroundWindow, SendMessageA, WM_IME_CONTROL},
    },
};

#[cfg(windows)]
fn imdisable() -> anyhow::Result<()> {
    unsafe {
        let fw = GetForegroundWindow();
        let ime = ImmGetDefaultIMEWnd(fw);
        let _res = SendMessageA(
            ime,
            WM_IME_CONTROL,
            WPARAM(IMC_SETOPENSTATUS as usize),
            LPARAM(0),
        );
    }
    Ok(())
}

use std::process::Command;

#[cfg(not(windows))]
fn imdisable() -> anyhow::Result<()> {
    Command::new("fcitx5-remote").arg("-c").spawn().ok();
    Ok(())
}
