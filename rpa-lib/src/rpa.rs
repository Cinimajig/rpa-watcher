use std::{fmt, io};

const GUID_LENGTH: usize = 36;
const SMALL_GUID_LENGTH: usize = 32;

/// Enum of RPA engings to watch.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(test, derive(Default))]
pub enum RpaEngine {
    #[cfg_attr(test, default)]
    ProcessRobot,
    #[serde(rename = "Power Automate")]
    PowerAutomate,
}

impl RpaEngine {
    /// Gets the process name of the RPA enginge.
    pub const fn process_name(&self) -> &'static str {
        match self {
            Self::ProcessRobot => "ProcessRobot.Process.exe",
            Self::PowerAutomate => "PAD.Robot.exe",
        }
    }

    /// Gets the process name of the RPA enginge.
    pub const fn process_name_lowercase(&self) -> &'static str {
        match self {
            Self::ProcessRobot => "processrobot.process.exe",
            Self::PowerAutomate => "pad.robot.exe",
        }
    }

    pub fn from_process_name(name: &str) -> Option<Self> {
        if name.eq_ignore_ascii_case(Self::ProcessRobot.process_name()) {
            Some(Self::ProcessRobot)
        } else if name.eq_ignore_ascii_case(Self::PowerAutomate.process_name()) {
            Some(Self::PowerAutomate)
        } else {
            None
        }
    }
}

impl fmt::Display for RpaEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.process_name())
    }
}

/// Trigger types for an RPA-process broken down to the most relevant ones.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(test, derive(Default))]
pub enum RpaTrigger {
    #[cfg_attr(test, default)]
    Attended,
    Unattended,
}

/// Collection of relevant data for the client to watch and send to the server.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(Default))]
pub struct RpaData {
    pub engine: RpaEngine,
    pub computer: String,
    pub instance: String,
    pub trigger: Option<RpaTrigger>,
    pub flow_id: Option<String>,
    pub parent_instance: Option<String>,
}

impl RpaData {
    pub fn from_cmdline(args: &str, hostname: &str) -> io::Result<Self> {
        let args = args.to_ascii_lowercase();

        // Find the process.
        let Some(index_exe) = args.find(".exe") else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "no process was found",
            ));
        };

        // Converts it to a RpaEngine if possible.
        let Some(engine) = args[1..index_exe + 4]
            .split('\\')
            .last()
            .and_then(RpaEngine::from_process_name)
        else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "no engine was found",
            ));
        };

        let run_id = match engine {
            RpaEngine::ProcessRobot => {
                match find_parameter(&args, "--instanceid=\"", GUID_LENGTH) {
                    Some(run_id) => run_id.to_string(),
                    None => {
                        return Err(io::Error::new(
                            io::ErrorKind::NotFound,
                            "instanceId was not found",
                        ))
                    }
                }
            }
            RpaEngine::PowerAutomate => {
                match find_parameter(&args, "--runid ", SMALL_GUID_LENGTH) {
                    Some(run_id) => format!(
                        "{}-{}-{}-{}-{}",
                        &run_id[..8],
                        &run_id[8..12],
                        &run_id[12..16],
                        &run_id[16..20],
                        &run_id[20..]
                    ),
                    None => {
                        return Err(io::Error::new(
                            io::ErrorKind::NotFound,
                            "runId was not found",
                        ))
                    }
                }
            }
        };

        // let env = match engine {
        //     RpaEngine::PowerAutomate => {
        //         find_parameter(&args, "--environmentname \"", GUID_LENGTH).map(|s| s.to_string())
        //     }
        //     // TODO! decode --serverBaseUriB64?
        //     RpaEngine::ProcessRobot => None,
        // };

        let flow_id = match engine {
            RpaEngine::PowerAutomate => find_parameter(&args, "--flowid ", SMALL_GUID_LENGTH).map(|s| format!(
                        "{}-{}-{}-{}-{}",
                        &s[..8],
                        &s[8..12],
                        &s[12..16],
                        &s[16..20],
                        &s[20..]
                    )),
            RpaEngine::ProcessRobot => None,
        };

        let trigger = match engine {
            RpaEngine::ProcessRobot => None,
            RpaEngine::PowerAutomate if args.contains("--trigger cloud") => Some(RpaTrigger::Unattended),
            RpaEngine::PowerAutomate => Some(RpaTrigger::Attended),
        };

        let parent_instance = match engine {
            RpaEngine::PowerAutomate => find_parameter(&args, "--rootFlowId ", SMALL_GUID_LENGTH).map(|s| format!(
                        "{}-{}-{}-{}-{}",
                        &s[..8],
                        &s[8..12],
                        &s[12..16],
                        &s[16..20],
                        &s[20..]
                    )),
            RpaEngine::ProcessRobot => None,
        };

        // let tenant_id = match engine {
        //     RpaEngine::PowerAutomate => find_parameter(&args, "--tenantid \"", GUID_LENGTH),
        //     // TODO! decode --serverBaseUriB64?
        //     RpaEngine::ProcessRobot => None,
        // };

        // let azure_data = match (flow_id, tenant_id) {
        //     (Some(f), Some(t)) => Some(AzureData {
        //         flow_id: f,
        //         tenant_id: t.to_string(),
        //     }),
        //     _ => None,
        // };

        // let env = match env {
        //     Some(e) if e.contains("one-drive") => None,
        //     Some(e) => Some(e),
        //     None => None,
        // };
        Ok(RpaData {
            engine,
            computer: hostname.to_string(),
            instance: run_id,
            flow_id,
            parent_instance,
            trigger,
            // azure_data,
        })
    }
}

fn find_parameter<'a>(cmdline_lc: &'a str, param: &'a str, length: usize) -> Option<&'a str> {
    cmdline_lc
        .find(param)
        .map(|i| i + param.len())
        .map(|index| &cmdline_lc[index..index + length])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_json() {
        let mut rpa_data: RpaData = RpaData::default();
        let json = serde_json::to_string(&rpa_data).unwrap();
        assert!(json.contains("\"engine\":\"ProcessRobot\""));

        rpa_data.engine = RpaEngine::PowerAutomate;
        let json = serde_json::to_string(&rpa_data).unwrap();
        assert!(json.contains("\"engine\":\"Power Automate\""));

        println!("{json}");

        let engine = serde_json::to_string(&rpa_data.engine).unwrap();
        assert_eq!(engine, "\"Power Automate\"");
    }

    #[test]
    fn parse() {
        // Auto generated GUIDs.
        let cmdline = r#""C:\Program Files (x86)\Power Automate Desktop\PAD.Robot.exe" --runId 9e0fc63338dd46e3b86fac1eceada33b --flowId 0d7d85e0c9744bd08bec545ef5d103af  --mode Run --trigger PadConsole --userpc --category PadConsole --correlationid "b367466d-4e80-44f8-b4b6-b0467d1d25a2" --environment "tip0" --environmentname "f7e54624-c28a-49f2-9da9-6f98ae509947" --geo "europe" --principaloid "e1b12f5e-d046-4679-ae35-c785a9d7766a" --principalpuid "1111111111111111" --region "westeurope" --sessionid "a24f7725-012c-4b3f-b55c-8ec8c1f92f1a" --tenantid "6d74b3cf-0246-4210-8b17-2042b0440806""#;

        RpaData::from_cmdline(cmdline, "localhost").unwrap();
    }

    #[test]
    fn to_and_from_json() {
        let data = RpaData {
            engine: RpaEngine::PowerAutomate,
            computer: "hostname".to_string(),
            instance: "rand-inst".to_string(),
            flow_id: Some("some-flow".to_string()),
            parent_instance: None,
            trigger: Some(RpaTrigger::default()),
        };

        let json = serde_json::to_string_pretty(&data);
        assert!(json.is_ok());

        assert!(serde_json::from_str::<RpaData>(&json.unwrap()).is_ok());
    }
}
