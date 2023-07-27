use device_query::{DeviceQuery, DeviceState, Keycode::LControl, Keycode::LeftBracket};
use std::{thread, time::Duration};
use windows::Win32::UI::Input::Ime::ImmDisableIME;

fn main() {
    let device_state = DeviceState::new();
    loop {
        let keys = device_state.get_keys();
        if keys.contains(&LControl) && keys.contains(&LeftBracket) {
            unsafe { ImmDisableIME(0u32) };
            thread::sleep(Duration::from_millis(500));
        }
    }
}
