use std::{fmt, io};

const GUID_LENGTH: usize = 36;
const SMALL_GUID_LENGTH: usize = 32;

/// Enum of RPA engings to watch.
#[derive(serde::Serialize, serde::Deserialize, Clone)]
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

/// Collection of relevant data for the client to watch and send to the server.
#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(Default))]
pub struct RpaData {
    pub pid: u32,
    pub engine: RpaEngine,
    pub computer: String,
    pub env: Option<String>,
    pub instance: String,
    pub azure_data: Option<AzureData>,
}

impl RpaData {
    pub fn from_cmdline(pid: u32, args: &str, hostname: &str) -> io::Result<Self> {
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

        let env = match engine {
            RpaEngine::PowerAutomate => {
                find_parameter(&args, "--environmentname \"", GUID_LENGTH).map(|s| s.to_string())
            }
            // TODO! decode --serverBaseUriB64?
            RpaEngine::ProcessRobot => None,
        };

        let flow_id = match engine {
            RpaEngine::PowerAutomate => find_parameter(&args, "--flowid ", SMALL_GUID_LENGTH)
                .and_then(|s| {
                    Some(format!(
                        "{}-{}-{}-{}-{}",
                        &s[..8],
                        &s[8..12],
                        &s[12..16],
                        &s[16..20],
                        &s[20..]
                    ))
                }),
            RpaEngine::ProcessRobot => None,
        };

        let tenant_id = match engine {
            RpaEngine::PowerAutomate => find_parameter(&args, "--tenantid \"", GUID_LENGTH),
            // TODO! decode --serverBaseUriB64?
            RpaEngine::ProcessRobot => None,
        };

        let azure_data = match (flow_id, tenant_id) {
            (Some(f), Some(t)) => Some(AzureData {
                flow_id: f,
                tenant_id: t.to_string(),
            }),
            _ => None,
        };

        Ok(RpaData {
            pid,
            engine,
            computer: hostname.to_string(),
            env,
            instance: run_id,
            azure_data,
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(Default))]
pub struct AzureData {
    pub flow_id: String,
    pub tenant_id: String,
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
        let pid: u32 = 1234;
        // Auto generated GUIDs.
        let cmdline = r#""C:\Program Files (x86)\Power Automate Desktop\PAD.Robot.exe" --runId 9e0fc63338dd46e3b86fac1eceada33b --flowId 0d7d85e0c9744bd08bec545ef5d103af  --mode Run --trigger PadConsole --userpc --category PadConsole --correlationid "b367466d-4e80-44f8-b4b6-b0467d1d25a2" --environment "tip0" --environmentname "f7e54624-c28a-49f2-9da9-6f98ae509947" --geo "europe" --principaloid "e1b12f5e-d046-4679-ae35-c785a9d7766a" --principalpuid "1111111111111111" --region "westeurope" --sessionid "a24f7725-012c-4b3f-b55c-8ec8c1f92f1a" --tenantid "269b0b7f-a757-4ae7-a732-4fba6c67faa6""#;

        RpaData::from_cmdline(pid, &cmdline, "localhost").unwrap();
    }

    #[test]
    fn from_json() {
        // TODO! Fix Deserializing.
        serde_json::from_str(r#"{
            "pid": 1234,
            "engine": "Power Automate",
            "computer": "Desktop",
            "env": "12312313",
            "instance": "sadsadasdasd",
            "azureData": null
          }"#).unwrap()
    }
}
