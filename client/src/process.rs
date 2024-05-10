use crate::handles::SafeHandle;
use std::{fs, mem, path, time::SystemTime};
use windows::{
    core::PWSTR, Wdk::System::Threading::*, Win32::{
        Foundation::*, Security::*, System::{
            Diagnostics::{Debug::*, ToolHelp::*},
            Threading::*,
        }
    }
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

/// Prototype.
#[allow(unused)]
pub fn get_name(handle: HANDLE) -> windows::core::Result<String> {
    unsafe {
        // Opens the token on the process.
        let mut token = SafeHandle::<true>(HANDLE(0));
        OpenProcessToken(handle, TOKEN_QUERY, &mut *token)?;

        // Retrieves the size of the user information. Error will be ignored.
        let mut size = 0;
        GetTokenInformation(*token, TokenUser, None, 0, &mut size).unwrap_or_default();

        // Retrieves the user information.
        // It has to be a buffer, to store the SID.
        let mut token_user_buffer = vec![0u8; size as usize];
        let token_user = token_user_buffer.as_ptr().cast::<TOKEN_USER>();
        let mut sid_use = SID_NAME_USE::default();
        GetTokenInformation(*token, TokenUser, Some(token_user_buffer.as_mut_ptr() as _), size, &mut size)?;
    
        
        // Retrieves the size of the username. Error will be ignored.
        let mut user_size = 0;
        let mut domain_size = 0;
        LookupAccountSidW(None, (*token_user).User.Sid, PWSTR::null(), &mut user_size, PWSTR::null(), &mut domain_size, &mut sid_use).unwrap_or_default();

        // Retrieves the username.
        let mut username = vec![0u16; user_size as usize + 1];
        let mut domain = vec![0u16; domain_size as usize + 1];
        LookupAccountSidW(None, (*token_user).User.Sid, PWSTR::from_raw(username.as_mut_ptr()), &mut user_size, PWSTR::from_raw(domain.as_mut_ptr()), &mut domain_size, &mut sid_use)?;

        // Converts to a string.
        Ok(String::from_utf16_lossy(&username[..user_size as usize]))
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

pub fn find_log_path(cmdline_lc: &str, flow_id: &str, instance: &str) -> Option<path::PathBuf> {
    const PARAM: &str = "--executionlogspath ";

    if flow_id.is_empty() {
        return None;
    }

    let Some(start_pos) = cmdline_lc.find(PARAM).map(|i| i + PARAM.len()) else {
        return None;
    };

    let mut short_args = &cmdline_lc[start_pos..];
    if short_args.starts_with('"') {
        let Some(last_index) = &short_args[1..].find('"') else {
            return None;
        };

        short_args = &short_args[1..*last_index + 1];
    }

    let mut log_path = path::PathBuf::from(short_args);
    log_path.push("Scripts");
    log_path.push(flow_id);
    log_path.push("Runs");
    log_path.push(instance);

    Some(log_path)
}

pub fn get_pad_name_and_action(
    mut path_run: path::PathBuf,
    name: &mut Option<String>,
    action: &mut Option<rpa::Action>,
) {
    path_run.push("RunDefinition.json");

    let Ok(json) = fs::read_to_string(&path_run) else {
        return;
    };

    let run_defination = serde_json::from_str::<rpa::RunDefinition>(&json).unwrap();

    *name = Some(run_defination.workflow.name);

    path_run.pop();
    path_run.push("Actions.log");

    let Ok(actions) = fs::read_to_string(path_run) else {
        return;
    };

    let Some(last_action) = actions.lines().last() else {
        return;
    };

    *action = serde_json::from_str::<rpa::Action>(last_action).ok();
}