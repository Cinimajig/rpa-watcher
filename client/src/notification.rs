use std::{fs, path::Path};

use crate::{env::NotificationType, shared_mem};
use windows::{core::*, Win32::{Foundation::*, UI::WindowsAndMessaging::*}};

fn find_window(parent: Option<HWND>, class: Option<&str>, title: Option<&str>) -> Result<HWND> {
    unsafe {
        let class: Vec<u16> = if let Some(class) = class { class.encode_utf16().chain(std::iter::once(0)).collect() } else { Vec::new() };
        let title: Vec<u16> = if let Some(title) = title { title.encode_utf16().chain(std::iter::once(0)).collect() } else { Vec::new() };
        let window = FindWindowExW(
            parent, 
            None, 
            if class.is_empty() { PCWSTR::null() } else { PCWSTR::from_raw(class.as_ptr()) }, 
            if title.is_empty() { PCWSTR::null() } else { PCWSTR::from_raw(title.as_ptr()) }
        )?;

        Ok(window)
    }
}

fn find_window_recursive(hieraci: &[String]) -> Option<HWND> {
        let mut window = HWND::default();

        for class in hieraci {
            if let Ok(handle) = find_window(Some(window), Some(class.as_str()), None) {
                window = handle;
            } else {
                return None;
            }
        }

        Some(window)
}

pub fn read_text(notitype: &mut NotificationType) -> Option<String> {
    match notitype {
        NotificationType::None => None,
        NotificationType::Window(vec) => read_window_text(vec),
        NotificationType::File(path_buf) => read_file(path_buf),
        NotificationType::SharedMemoryA(memmap, name) => shared_mem::read_shared_mem(memmap, name),
        NotificationType::SharedMemoryW(memmap, name) => shared_mem::read_shared_mem_wide(memmap, name),
    }
}

fn read_file(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok()
}

fn read_window_text(hieraci: &[String]) -> Option<String> {
    unsafe {
        // Finds window/handle.
        let handle = find_window_recursive(hieraci)?;
        // Retrieves text size.
        let size = SendMessageW(handle, WM_GETTEXTLENGTH, None, None).0 as usize;
        let mut buffer = vec![0u16; size + 1];
        // Retrieves the text.
        let size = SendMessageW(handle, WM_GETTEXT, Some(WPARAM(buffer.len())), Some(LPARAM(buffer.as_mut_ptr() as _))).0 as usize;
        Some(String::from_utf16_lossy(&buffer[..size]))
    }
}
