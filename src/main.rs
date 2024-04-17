use::registry::{Hive, Security};
use windows::core::{PCSTR, PCWSTR};
use windows::Win32::Foundation::{COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, SYSTEMTIME, WPARAM};
use windows::Win32::Globalization::{DATE_LONGDATE, GetDateFormatEx, GetTimeFormatEx, TIME_FORMAT_FLAGS};
use windows::Win32::Graphics::Gdi::{BeginPaint, COLOR_WINDOW, CreateSolidBrush, DrawTextW, DT_CENTER, DT_VCENTER, FillRect, GetSysColor, InvalidateRect, PAINTSTRUCT};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::{BS_PUSHBUTTON, CreateWindowExA, CS_HREDRAW, CS_VREDRAW, DefWindowProcA, DispatchMessageA, GetMessageA, HMENU, IDC_ARROW, LoadCursorW, PostQuitMessage, RegisterClassA, ShowWindow, SW_SHOW, TranslateMessage, WINDOW_STYLE, WM_COMMAND, WM_CREATE, WM_DESTROY, WM_PAINT, WNDCLASSA, WS_CHILD, WS_OVERLAPPEDWINDOW, WS_VISIBLE};
use once_cell::unsync::Lazy;

static mut TEXT_UTF16: Lazy<Vec<u16>> = Lazy::new(|| get_output_string().encode_utf16().collect());

fn main() {
    create_window();
}


fn get_registry_values_from_current_user() -> String {
    let regkey = Hive::CurrentUser
        .open("Control Panel\\International", Security::Read)
        .unwrap();

    return  format!(
        "Dateformat: {}, Timeformat: {}",
        regkey.value("sLongDate").unwrap(),
        regkey.value("sTimeFormat").unwrap(),
    );
}

fn get_time_format() -> String {
    unsafe {
        let time_buffer_size = GetTimeFormatEx(
            None,
            TIME_FORMAT_FLAGS(0),
            None,
            PCWSTR::null(),
            None);

        let mut time_buffer: Vec<u16> = vec![0; time_buffer_size as usize];

        let test = GetTimeFormatEx(
            None,
            TIME_FORMAT_FLAGS(0),
            None,
            None,
            Some(&mut time_buffer[..]));

        return String::from_utf16_lossy(&time_buffer);
    }
}

fn get_date_long_format() -> String {
    unsafe {
        let buffer_length = GetDateFormatEx(
            None,
            DATE_LONGDATE,
            None,
            PCWSTR::null(),
            None,
            None);

        let mut buffer: Vec<u16> = vec![0; buffer_length as usize];

        GetDateFormatEx(
            None,
            DATE_LONGDATE,
            None,
            PCWSTR::null(),
            Some(&mut buffer[..]),
            None);

        return String::from_utf16_lossy(&buffer);
    }

}

fn get_output_string() -> String {
    return format!("Current date: {} Current time: {} \r\n Equals Registry Settings: {}",
                   get_date_long_format(),
                   get_time_format(),
                   get_registry_values_from_current_user());
}


unsafe extern "system" fn window_procedure(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {

    match message {
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(window, &mut ps);
            let rect = &mut ps.rcPaint;

            let background_brush = CreateSolidBrush(COLORREF(GetSysColor(COLOR_WINDOW)));
            FillRect(hdc, rect, background_brush);

            DrawTextW(hdc, &mut *TEXT_UTF16, rect,  DT_CENTER | DT_VCENTER);
            LRESULT(0)
        }
        WM_CREATE => {
            CreateWindowExA(
                Default::default(),
                PCSTR(b"BUTTON\0".as_ptr()),
                PCSTR(b"Refresh\0".as_ptr()),
                WS_VISIBLE | WS_CHILD | WINDOW_STYLE(BS_PUSHBUTTON as u32),
                600,
                125,
                60,
                30,
                window,
                HMENU(21),
                GetModuleHandleA(None).unwrap(),
                None);
            LRESULT(0)
        }
        WM_COMMAND => {
            if (wparam.0 as u32) == 21 {
                println!("{}", "Refreshing ...");
                TEXT_UTF16= Lazy::new(|| get_output_string().encode_utf16().collect());
                InvalidateRect(window, None, true);
            }
            LRESULT(0)
        }
        WM_DESTROY => unsafe {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcA(window, message, wparam, lparam),
    }
}

fn create_window() -> windows::core::Result<HWND> {
    unsafe {
        let wc = WNDCLASSA {
            hInstance:  HINSTANCE::from(GetModuleHandleA(None).unwrap()),
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            lpszClassName: PCSTR("window_class".as_ptr()),
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_procedure),
            ..Default::default()
        };

        RegisterClassA(&wc);

        let window = CreateWindowExA(
            Default::default(),
            PCSTR("window_class".as_ptr()),
            PCSTR("Demo OS Date & Time".as_ptr()),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            200,
            200,
            800,
            200,
            None,
            None,
            GetModuleHandleA(None).unwrap(),
            None
        );

        let mut message = std::mem::zeroed();
        while GetMessageA(&mut message, None, 0, 0).into() {
            TranslateMessage(&message);
            DispatchMessageA(&message);
        }

        ShowWindow(window, SW_SHOW);
        Ok(window)
    }
}

