mod dbg;
mod env;
mod handles;
mod http;
mod privilage;
mod process;

use dbg::*;
use rpa::*;
use std::{convert::Infallible, io, thread, time};
use windows::core::*;

const RPA_PROCESSES: &[&str] = &[
    RpaEngine::PowerAutomate.process_name(),
    RpaEngine::ProcessRobot.process_name(),
];

fn main() -> io::Result<Infallible> {
    let env = env::Environment::from_file_then_env();
    dbg_output(format!("<RPA.Watcher> {env:?}"));

    // Enable debug privilages.
    privilage::enable_debug_priv()?;

    // Get the hostname.
    let comp_name: String = get_hostname()?;

    // A vector of running processes/flows.
    let mut items: Vec<RpaData> = Vec::with_capacity(10);

    dbg_output("<RPA.Watcher> Initialization complete.");

    loop {
        let seconds_to_sleep = 'process_lookup: {
            // Get a handle and pid to all relevant processes.
            let processes = process::find_processes(RPA_PROCESSES)?;
            if processes.is_empty() {
                dbg_output("<RPA.Watcher> No process(es) was found.");
                break 'process_lookup SLEEP_SECONDS_SHORT;
            }

            // Get's the commandline for each and constructs an RpaData structure.
            for process in processes {
                let Ok(cmdline) = process::get_cmdline(*process) else {
                    continue;
                };

                let started = process::get_started_time(*process).ok();

                let Ok(rpa_data) = RpaData::from_cmdline(&cmdline, &comp_name, started) else {
                    continue;
                };

                items.push(rpa_data);
            }

            match http::post(&env.url, &env.token, &items) {
                Ok(_) => (),
                Err(err) => dbg_output(err),
            }

            items.clear();
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
