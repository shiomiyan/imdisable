use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::UI::Input::Ime::ImmGetDefaultIMEWnd;
use windows::Win32::UI::Input::Ime::IMC_SETOPENSTATUS;
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
use windows::Win32::UI::WindowsAndMessaging::SendMessageA;
use windows::Win32::UI::WindowsAndMessaging::WM_IME_CONTROL;

fn main() -> windows::core::Result<()> {
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
