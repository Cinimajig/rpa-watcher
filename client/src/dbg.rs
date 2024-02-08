use std::{fmt, iter};
use windows::{core::PCWSTR, Win32::System::Diagnostics::Debug::OutputDebugStringW};

pub fn dbg_output(text: impl fmt::Display) {
    #[cfg(debug_assertions)]
    println!("{text}");

    #[cfg(not(debug_assertions))]
    unsafe {
        let dbg: Box<[u16]> = text.to_string().encode_utf16().chain(iter::once(0)).collect();
        OutputDebugStringW(PCWSTR::from_raw(dbg.as_ptr()));
    }
}
