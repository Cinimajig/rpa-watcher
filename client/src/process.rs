use crate::handles::SafeHandle;
use std::{mem, time::SystemTime};
use windows::{
    Wdk::System::Threading::*,
    Win32::{
        Foundation::*,
        System::{
            Diagnostics::{Debug::*, ToolHelp::*},
            Threading::*,
        },
    },
};

/// Get's the commandline of the process handle.
/// If the handle is invalid, the function will fail.
pub fn get_cmdline(process: HANDLE) -> windows::core::Result<String> {
    unsafe {
        // Finds the PEB of the process.
        let mut pbi = PROCESS_BASIC_INFORMATION::default();
        NtQueryInformationProcess(
            process,
            ProcessBasicInformation,
            &mut pbi as *mut _ as _,
            mem::size_of::<PROCESS_BASIC_INFORMATION>() as _,
            std::ptr::null_mut(),
        )
        .ok()?;

        // Reads the PEB to get the parameters.
        let mut peb = PEB::default();
        ReadProcessMemory(
            process,
            pbi.PebBaseAddress as _,
            &mut peb as *mut _ as _,
            mem::size_of::<PEB>() as _,
            None,
        )?;

        // Reads the parameters, to find the commandline.
        let mut process_parameters = RTL_USER_PROCESS_PARAMETERS::default();
        ReadProcessMemory(
            process,
            peb.ProcessParameters as _,
            &mut process_parameters as *mut _ as _,
            mem::size_of::<RTL_USER_PROCESS_PARAMETERS>(),
            None,
        )?;

        // Reads the commandline.
        let mut buffer = vec![0u16; process_parameters.CommandLine.Length as _];
        ReadProcessMemory(
            process,
            mem::transmute(process_parameters.CommandLine.Buffer),
            buffer.as_mut_ptr() as _,
            buffer.len(),
            None,
        )?;

        // Converts the buffer to a UTF-8 String.
        let size = (0usize..).take_while(|i| buffer[*i] != 0).count();
        Ok(String::from_utf16_lossy(&buffer[..size]))
    }
}

/// Finds the given process names and returns a handle to it.
/// The handle is wrapped in a [`SafeHandle`] that automatically
/// closes it when dropped.
pub fn find_processes(files: &[&str]) -> windows::core::Result<Vec<SafeHandle>> {
    let mut handles: Vec<SafeHandle> = Vec::with_capacity(10);

    unsafe {
        // Using the ASCII version, because we don't need Unicode support right here.
        let mut entry = PROCESSENTRY32 {
            dwSize: mem::size_of::<PROCESSENTRY32>() as _,
            ..Default::default()
        };
        let snapshot = SafeHandle::<true>(CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?);

        // First process is always PID 0 or 4 (System Processes). it's needed for Process32Next.
        Process32First(*snapshot, &mut entry)?;

        // Iterate over all other processes.
        while Process32Next(*snapshot, &mut entry).is_ok() {
            // let size = (0usize..).take_while(|i| entry.szExeFile[*i] != 0).count();
            // let name = &entry.szExeFile[..size];
            let name = std::ffi::CStr::from_ptr(entry.szExeFile.as_ptr());

            for file in files {
                if file.as_bytes().eq_ignore_ascii_case(name.to_bytes()) {
                    let process =
                        SafeHandle(OpenProcess(PROCESS_ALL_ACCESS, false, entry.th32ProcessID)?);
                    handles.push(process);
                    break;
                }
            }
        }

        Ok(handles)
    }
}

pub fn get_started_time(handle: HANDLE) -> windows::core::Result<rpa::DateTime> {
    unsafe {
        let (mut ctime, mut _etime, mut _ktime, mut _utime) = Default::default();
        GetProcessTimes(handle, &mut ctime, &mut _etime, &mut _ktime, &mut _utime)?;
        
        let systime = rpa::DateTime::from(mem::transmute::<FILETIME, SystemTime>(ctime));

        Ok(systime)
    }
} 