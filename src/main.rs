/*#![cfg(windows)]
#![windows_subsystem = "windows"]*/

use std::mem;
use windows::{
    core::*,
    Win32::{
        Foundation::SYSTEMTIME,
        System::SystemInformation::*,
        UI::{
            WindowsAndMessaging::*
        },
        Globalization::{
            GetTimeFormatEx,
            GetDateFormatEx,
            DATE_LONGDATE,
            TIME_FORMAT_FLAGS
        },
    },
};
use registry::{
    Hive,
    Security
};
use windows::Win32::Foundation::{COLORREF, HINSTANCE, HMODULE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::{BeginPaint, COLOR_BACKGROUND, COLOR_WINDOW, CreateSolidBrush, DrawTextW, DT_CENTER, DT_SINGLELINE, DT_VCENTER, DT_WORDBREAK, FillRect, GetSysColor, InvalidateRect, PAINTSTRUCT, RGBQUAD};
use windows::Win32::System::LibraryLoader::{GetModuleHandleA};
use once_cell::unsync::Lazy;

static mut DISPLAY_TEXT : String = String::new();
static mut TEXT_UTF16: Lazy<Vec<u16>> = Lazy::new(|| get_output_string().encode_utf16().collect());



fn main() {
    unsafe {
        let date = get_date_long_format();
        let time = get_time_format();
        let date_time_format = get_registry_values_from_current_user();

        let display_string = format!("Current date: {} Current time: {} \r\n Equals Registry Settings: {}",
                                     &date,
                                     &time,
                                     &date_time_format
        );
        println!("{}",display_string);

        create_window();

    }

}

fn get_output_string() -> String {
    unsafe {
    return format!("Current date: {} Current time: {} \r\n Equals Registry Settings: {}",
                   get_date_long_format(),
                   get_time_format(),
                   get_registry_values_from_current_user());
        }
}

fn create_window() -> Result<HWND> {
    unsafe {
        let instance = GetModuleHandleA(None).unwrap();
        debug_assert!(instance.0 != 0);

        let class_name = PCSTR("window_class".as_ptr());

        let wc = WNDCLASSA {
            hInstance: HINSTANCE::from(instance),
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            lpszClassName: PCSTR(class_name.as_ptr()),
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            ..Default::default()
        };

        let reg = RegisterClassA(&wc);
        debug_assert!(reg != 0);

        let window = CreateWindowExA(
            Default::default(),
            class_name,
            PCSTR("Demo OS Date & Time".as_ptr()),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            200,
            200,
            800,
            200,
            None,
            None,
            instance,
            None
        );

        let mut message = std::mem::zeroed();
        while GetMessageA(&mut message, None, 0, 0).into() {
            TranslateMessage(&message);
            DispatchMessageA(&message);
        }

        ShowWindow(window, SW_SHOW);

        create_button(window);

        Ok(window)
    }
}

unsafe fn create_button(window: HWND) -> Result<HWND> {
    let refresh_button = CreateWindowExA(
        Default::default(),
        PCSTR("BUTTON".as_ptr()),
        PCSTR("Refresh".as_ptr()),
        WS_TABSTOP | WS_VISIBLE | WS_CHILD | WINDOW_STYLE(BS_DEFPUSHBUTTON as u32),
        10,
        10,
        10,
        10,
        window,
        None, // Button ID
        GetModuleHandleA(None).unwrap(),
        None,
    );
        Ok(refresh_button)
}


unsafe fn get_date_long_format() -> String {
    let mut system_time : SYSTEMTIME = mem::zeroed();
    let system_time = GetLocalTime();

    let buffer_length = GetDateFormatEx(
        Option::None,
        DATE_LONGDATE,
        Option::from(&system_time as *const SYSTEMTIME),
        PCWSTR::null(),
        Option::None,
        Option::None);

    //let buffer : &mut [u16] = &mut [];
    let mut buffer: Vec<u16> = vec![0; buffer_length as usize];



    GetDateFormatEx(
        Option::None,
        DATE_LONGDATE,
        Option::from(&system_time as *const SYSTEMTIME),
        PCWSTR::null(),
        Some(&mut buffer[..]),
        Option::None);

    println!("{}", String::from_utf16_lossy(&buffer));

    return String::from_utf16_lossy(&buffer);

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
        let mut system_time : SYSTEMTIME = mem::zeroed();
        let system_time = GetLocalTime();
        const LOCALE_NAME_USER_DEFAULT: PCWSTR = PCWSTR::null();


        let time_buffer_size = GetTimeFormatEx(
            Option::None,
            TIME_FORMAT_FLAGS(0),
            Option::from(&system_time as *const SYSTEMTIME),
            PCWSTR::null(),
            Option::None);

        let mut time_buffer: Vec<u16> = vec![0; time_buffer_size as usize];

        let time_buffer_size = GetTimeFormatEx(
            Option::None,
            TIME_FORMAT_FLAGS(0),
            Option::from(&system_time as *const SYSTEMTIME),
            Option::None,
            Some(&mut time_buffer[..]));



        println!("{}", String::from_utf16_lossy(&time_buffer));

        return String::from_utf16_lossy(&time_buffer);
    }
}


unsafe extern "system" fn window_proc(
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
                50,
                25,
                window,
                HMENU(21), // Cast ID to HMENU
                GetModuleHandleA(None).unwrap(),
                None);
            LRESULT(0)
        }
        WM_COMMAND => {
            if (wparam.0 as u32) == 21 {
                println!("{}", "Refreshing ...");
                TEXT_UTF16 = mem::zeroed();
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
