use std::io;
use sharedef::rpa::*;

pub fn from_args(pid: u32, args: &str, hostname: &str) -> io::Result<RpaData> {
    let args = args.to_ascii_lowercase();

    // Find the process.
    let Some(index_exe) = args.find(".exe") else {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "no process was found"));
    };

    // Converts it to a RpaEngine if possible.
    let Some(engine) = args[1..index_exe + 4].split('\\').last().and_then(RpaEngine::from_process_name) else {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "no engine was found")); 
    };

    let run_id = match engine {
        RpaEngine::ProcessRobot => match find_parameter(&args, "--instanceid=\"", 36) {
            Some(run_id) => run_id.to_string(),
            None => return Err(io::Error::new(io::ErrorKind::NotFound, "instanceId was not found")),
        },
        RpaEngine::PowerAutomate => match find_parameter(&args, "--runid ", 32) {
            Some(run_id) => format!("{}-{}-{}-{}-{}", &run_id[..8], &run_id[8..12], &run_id[12..16], &run_id[16..20], &run_id[20..]),
            None => return Err(io::Error::new(io::ErrorKind::NotFound, "runId was not found")),
        },
    };

    


    todo!();
}

fn find_parameter<'a>(cmdline_lc: &'a str, param: &'a str, length: usize) -> Option<&'a str> {
    match cmdline_lc.find(param).map(|i| i + param.len()) {
        Some(index) => Some(&cmdline_lc[index..index + length]),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        let pid: u32 = 1234;
        // Auto generated GUIDs.
        let cmdline = r#""C:\Program Files (x86)\Power Automate Desktop\PAD.Robot.exe" --runId 9e0fc63338dd46e3b86fac1eceada33b --flowId 0d7d85e0c9744bd08bec545ef5d103af  --mode Run --trigger PadConsole --userpc --category PadConsole --correlationid "b367466d-4e80-44f8-b4b6-b0467d1d25a2" --environment "tip0" --environmentname "f7e54624-c28a-49f2-9da9-6f98ae509947" --geo "europe" --principaloid "e1b12f5e-d046-4679-ae35-c785a9d7766a" --principalpuid "1111111111111111" --region "westeurope" --sessionid "a24f7725-012c-4b3f-b55c-8ec8c1f92f1a" --tenantid "269b0b7f-a757-4ae7-a732-4fba6c67faa6""#;

        from_args(pid, &cmdline, "localhost").unwrap();
    }
}