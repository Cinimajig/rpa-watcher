mod handles;
mod privilage;
mod process;
mod parser;

use std::{convert::Infallible, thread, time};
use sharedef::{*, rpa::*};
use windows::core::*;

const RPA_PROCESSES: &[&str] = &[
    RpaEngine::PowerAutomate.process_name(), 
    RpaEngine::ProcessRobot.process_name(),
];

fn main() -> Result<Infallible> {
    privilage::enable_debug_priv()?;

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

                let Ok(rpa_data) = parser::from_str(&cmdline) else {
                    continue;
                };
            }

            SLEEP_SECONDS_LONG
        };

        thread::sleep(time::Duration::from_secs(seconds_to_sleep));
    }
}
