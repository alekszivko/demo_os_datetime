use std::mem;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use windows::core::*;
use windows::Win32::Foundation::SYSTEMTIME;
use windows::Win32::Globalization::{GetTimeFormatEx, TIME_NOSECONDS, GetDateFormatEx, DATE_AUTOLAYOUT, DATE_SHORTDATE, LOCALE_NAME_SYSTEM_DEFAULT, GetDateFormatW, DATE_LONGDATE, GetLocaleInfoEx, LOCALE_SNAME, TIME_FORMAT_FLAGS};
use windows::Win32::System::SystemInformation::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use registry::{
    Hive,
    Security
};
use windows::Win32::System::SystemServices::LOCALE_NAME_MAX_LENGTH;

fn main() {
    unsafe {
        // Get Date Time Format Settings From Current User

        // Hive that contains required infos
        let regkey = Hive::CurrentUser
            .open("Control Panel\\International", Security::Read)
            .unwrap();

        //current time
        let mut system_time : SYSTEMTIME = mem::zeroed();
        let system_time = GetLocalTime();
        const LOCALE_NAME_USER_DEFAULT: PCWSTR = PCWSTR::null();
        //pub const LOCALE_NAME_USER_DEFAULT: PCWSTR = w!("!x-sys-default-locale");

        //GetLocaleInfoEx(LOCALE_NAME_USER_DEFAULT, LOCALE_SNAME, )

        let buffer_length = GetDateFormatEx(
            Option::None,
            DATE_LONGDATE,
            Option::from(&system_time as *const SYSTEMTIME),
            PCWSTR::null(),
            Option::None,
            Option::None);

        //let buffer : &mut [u16] = &mut [];
        let mut buffer: Vec<u16> = vec![0; buffer_length as usize];

        let locale = format!("{}", regkey.value("LocaleName").unwrap());
        let locale2 = "en-US\0".encode_utf16();



        GetDateFormatEx(
            Option::None,
            DATE_LONGDATE,
            Option::from(&system_time as *const SYSTEMTIME),
            PCWSTR::null(),
            Some(&mut buffer[..]),
            Option::None);


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


        println!("{}", String::from_utf16_lossy(&buffer));
        println!("{}", String::from_utf16_lossy(&time_buffer));






        //date time formats - only shortform

        let date_time_format = format!(
            "Dateformat: {}, Timeformat: {}",
            regkey.value("sShortDate").unwrap(),
            regkey.value("sTimeFormat").unwrap(),
        );

        let dt_local: DateTime<Local> = SystemTime::now().into();

        println!("{}", dt_local);

        //println!("{}", String::from(&date));

        // TODO use long form as well ?

        println!("{}", date_time_format);


        MessageBoxA(
            None,
            PCSTR(date_time_format.as_ptr()),
            s!("Date Time Demo"),
            MB_OK,
        );
    }
}
