#![allow(static_mut_refs)]

use std::ptr;

use windows::{
    core::{PCSTR, PCWSTR},
    Win32::{Foundation::*, System::Memory::*},
};

pub fn read_shared_mem(memmap: &mut Option<SharedMemMap>, name: &str) -> Option<String> {
    unsafe {
        if memmap.is_none() {
            let mut map = SharedMemMap::new();
            map.handle =
                OpenFileMappingA(FILE_MAP_ALL_ACCESS.0, false, PCSTR::from_raw(name.as_ptr()))
                    .ok()?;

            map.ptr = MapViewOfFile(
                map.handle,
                FILE_MAP_ALL_ACCESS,
                0,
                0,
                0, // 0 == Until end of buffer
            );
            *memmap = Some(map);
        }

        match &memmap {
            Some(map) => Some(PCSTR::from_raw(map.address()).display().to_string()),
            None => None,
        }
    }
}

pub fn read_shared_mem_wide(memmap: &mut Option<SharedMemMap>, name: &[u16]) -> Option<String> {
    unsafe {
        if memmap.is_none() {
            let mut map = SharedMemMap::new();
            map.handle = OpenFileMappingW(
                FILE_MAP_ALL_ACCESS.0,
                false,
                PCWSTR::from_raw(name.as_ptr()),
            )
            .ok()?;

            map.ptr = MapViewOfFile(
                map.handle,
                FILE_MAP_ALL_ACCESS,
                0,
                0,
                0, // 0 == Until end of buffer
            );
            *memmap = Some(map);
        }

        match &memmap {
            Some(map) => Some(PCWSTR::from_raw(map.address()).display().to_string()),
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct SharedMemMap {
    handle: HANDLE,
    ptr: MEMORY_MAPPED_VIEW_ADDRESS,
}

impl SharedMemMap {
    pub fn new() -> Self {
        Self {
            handle: HANDLE(0 as _),
            ptr: MEMORY_MAPPED_VIEW_ADDRESS {
                Value: ptr::null_mut(),
            },
        }
    }

    pub fn address<T>(&self) -> *mut T {
        self.ptr.Value as _
    }
}

impl Drop for SharedMemMap {
    fn drop(&mut self) {
        unsafe {
            let _ = UnmapViewOfFile(self.ptr);
            let _ = CloseHandle(self.handle);
        }
    }
}
