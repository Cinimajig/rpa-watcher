use std::ops::{Deref, DerefMut};
use windows::Win32::Foundation::*;

pub const DEBUG_MODE: bool = cfg!(debug_assertions);

/// A Wrapper type, that captures a `HANDLE` and then calls `CloseHandle` when dropped.
///
/// If the handle shouldn't call `CloseHandle`, then don't use this struct.
#[repr(transparent)]
pub struct SafeHandle<const SUPRESS: bool = { !DEBUG_MODE }>(pub HANDLE);

impl<const SUPRESS: bool> Drop for SafeHandle<SUPRESS> {
    fn drop(&mut self) {
        unsafe {
            if !self.0.is_invalid() {
                CloseHandle(self.0).unwrap_or_default();
                if !SUPRESS {
                    println!("[!] SafeHandle::drop({})", self.0.0 as isize);
                }
            }
        }
    }
}

impl<const SUPRESS: bool> Deref for SafeHandle<SUPRESS> {
    type Target = HANDLE;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const SUPRESS: bool> DerefMut for SafeHandle<SUPRESS> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
