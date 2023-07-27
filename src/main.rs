use device_query::{DeviceQuery, DeviceState, Keycode::LControl, Keycode::LeftBracket};
use std::{thread, time::Duration};
use windows::Win32::{
    Foundation::{LPARAM, WPARAM},
    UI::{
        Input::Ime::{ImmDisableIME, ImmGetDefaultIMEWnd, IMC_SETOPENSTATUS},
        WindowsAndMessaging::{GetForegroundWindow, SendMessageA, WM_IME_CONTROL},
    },
};

fn main() {
    let device_state = DeviceState::new();
    loop {
        let keys = device_state.get_keys();
        if keys.contains(&LControl) && keys.contains(&LeftBracket) {
            imdisable().ok();
            thread::sleep(Duration::from_millis(500));
        }
    }
}

fn imdisable() -> anyhow::Result<()> {
    unsafe {
        let fw = GetForegroundWindow();
        let ime = ImmGetDefaultIMEWnd(fw);
        let res = SendMessageA(
            ime,
            WM_IME_CONTROL,
            WPARAM(IMC_SETOPENSTATUS as usize),
            LPARAM(0),
        );
        dbg!(&res);
    }
    Ok(())
}
