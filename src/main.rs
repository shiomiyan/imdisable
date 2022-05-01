use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::UI::Input::Ime::ImmGetDefaultIMEWnd;
use windows::Win32::UI::Input::Ime::IMC_SETOPENSTATUS;
use windows::Win32::UI::Input::KeyboardAndMouse;
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
use windows::Win32::UI::WindowsAndMessaging::SendMessageA;
use windows::Win32::UI::WindowsAndMessaging::WM_IME_CONTROL;

fn main() {
    let mut hk = hotkey::Listener::new();
    hk.register_hotkey(
        hotkey::modifiers::CONTROL,
        KeyboardAndMouse::VK_OEM_4.0 as u32,
        || {
            println!("Ctrl-[ pressed.");
            imdisable().expect("WARN: couldn't disable ime.");
        },
    )
    .unwrap();

    hk.listen();
}

fn imdisable() -> windows::core::Result<()> {
    unsafe {
        let fw = GetForegroundWindow();
        let ime = ImmGetDefaultIMEWnd(fw);

        let res = SendMessageA(
            ime,
            WM_IME_CONTROL,
            WPARAM(IMC_SETOPENSTATUS as usize),
            LPARAM(0),
        );

        println!("{:?}", res);
    }
    Ok(())
}
