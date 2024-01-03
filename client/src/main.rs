mod handles;
mod privilage;
mod process;
mod parser;
mod http;

use std::{convert::Infallible, thread, time};
use sharedef::{*, rpa::*};
use windows::core::*;

const RPA_PROCESSES: &[&str] = &[
    RpaEngine::PowerAutomate.process_name(), 
    RpaEngine::ProcessRobot.process_name(),
];

fn main() -> Result<Infallible> {
    privilage::enable_debug_priv()?;

    let comp_name: String = get_hostname()?;

    loop {
        let seconds_to_sleep = 'process_lookup: {
            let processes = process::find_processes(RPA_PROCESSES)?;
            if processes.len() == 0 {
                break 'process_lookup SLEEP_SECONDS_SHORT;
            }

            for (process, pid) in processes {
                let Ok(cmdline) = process::get_cmdline(*process) else {
                    continue;
                };

                let Ok(rpa_data) = parser::from_args(pid, &cmdline, &comp_name) else {
                    continue;
                };

                serde_json::to_string(&rpa_data)
            }

            SLEEP_SECONDS_LONG
        };

        thread::sleep(time::Duration::from_secs(seconds_to_sleep));
    }
}

fn get_hostname() -> Result<String> {
    use windows::Win32::System::WindowsProgramming::*;

    unsafe {
        let mut buffer = [0u16; MAX_COMPUTERNAME_LENGTH as usize + 1];
        let mut size = MAX_COMPUTERNAME_LENGTH + 1;
        GetComputerNameW(PWSTR::from_raw(buffer.as_mut_ptr()), &mut size)?;

        Ok(String::from_utf16_lossy(&buffer[..size as usize]))
    }
}
