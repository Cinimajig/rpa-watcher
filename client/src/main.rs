#![cfg_attr(feature = "windows", windows_subsystem = "windows")]
mod dbg;
mod env;
mod handles;
mod http;
mod privilage;
mod process;
mod notification;
mod shared_mem;

use dbg::*;
use rpa::*;
use std::{convert::Infallible, io, thread, time};
use windows::core::*;

const RPA_PROCESSES: &[&str] = &[
    RpaEngine::PowerAutomateV2.process_name(),
    RpaEngine::PowerAutomate.process_name(),
    RpaEngine::ProcessRobot.process_name(),
];

fn main() -> io::Result<Infallible> {
    let mut env = env::Environment::from_file_then_env();
    dbg_output(format!("<RPA.Watcher> {env:?}"));

    let dump = std::env::args()
        .skip(1)
        .collect::<String>()
        .to_ascii_lowercase()
        == "dump";

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

                let Ok(mut rpa_data) = RpaData::from_cmdline(&cmdline, &comp_name, started) else {
                    continue;
                };

                // let username = match process::get_name(*process) {
                //     Ok(s) => s,
                //     Err(err) => {
                //         dbg_output(format!("<RPA.Watcher> Failed to get username. {err}"));
                //         items.push(rpa_data);
                //         continue;
                //     },
                // };

                'name_and_action: {
                    if rpa_data.engine == RpaEngine::PowerAutomateV2 || rpa_data.engine == RpaEngine::PowerAutomate {
                        let Some(path_run) = process::find_log_path(
                            &cmdline,
                            &rpa_data.flow_id.clone().unwrap_or_default(),
                            &rpa_data.instance,
                        ) else {
                            break 'name_and_action;
                        };

                        if !path_run.is_dir() {
                            dbg_output(format!(
                                "<RPA.Watcer> Can't find Directory: {}.",
                                path_run.to_string_lossy()
                            ));
                            break 'name_and_action;
                        }

                        process::get_pad_name_and_action(
                            path_run,
                            &mut rpa_data.name,
                            &mut rpa_data.action,
                        );
                    }
                }

                rpa_data.notification = notification::read_text(&mut env.notification);

                items.push(rpa_data);
            }

            if dump {
                println!(":::");
                println!("{}", serde_json::to_string_pretty(&items).unwrap_or_else(|err| err.to_string()));
                println!(":::");
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
        GetComputerNameW(Some(PWSTR::from_raw(buffer.as_mut_ptr())), &mut size)?;

        Ok(String::from_utf16_lossy(&buffer[..size as usize]))
    }
}
