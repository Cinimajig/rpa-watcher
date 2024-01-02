use crate::handles::SafeHandle;
use std::mem;
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
            mem::transmute(&mut pbi),
            mem::size_of::<PROCESS_BASIC_INFORMATION>() as _,
            std::ptr::null_mut(),
        )
        .ok()?;

        // Reads the PEB to get the parameters.
        let mut peb = PEB::default();
        ReadProcessMemory(
            process,
            pbi.PebBaseAddress as _,
            mem::transmute(&mut peb),
            mem::size_of::<PEB>() as _,
            None,
        )?;

        // Reads the parameters, to find the commandline.
        let mut process_parameters = RTL_USER_PROCESS_PARAMETERS::default();
        ReadProcessMemory(
            process,
            peb.ProcessParameters as _,
            mem::transmute(&mut process_parameters),
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
pub fn find_processes(files: &[&str]) -> windows::core::Result<Vec<(SafeHandle, u32)>> {
    let mut handles: Vec<(SafeHandle, u32)> = Vec::with_capacity(10);

    unsafe {
        let mut entry = PROCESSENTRY32 {
            dwSize: mem::size_of::<PROCESSENTRY32>() as _,
            ..Default::default()
        };
        let snapshot = SafeHandle::<true>(CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?);

        // First process is always PID 0 (System Process). it's needed for Process32Next.
        // Not using Wide-functions, since the manifest forces UTF-8.
        Process32First(*snapshot, &mut entry)?;

        // Iterate over all other processes.
        while let Ok(_) = Process32Next(*snapshot, &mut entry) {
            let size = (0usize..).take_while(|i| entry.szExeFile[*i] != 0).count();
            let name = &entry.szExeFile[..size];

            for file in files {
                if file.as_bytes().eq_ignore_ascii_case(name) {
                    let process =
                        SafeHandle(OpenProcess(PROCESS_ALL_ACCESS, false, entry.th32ProcessID)?);
                    handles.push((process, entry.th32ProcessID));
                    break;
                }
            }
        }

        Ok(handles)
    }
}
