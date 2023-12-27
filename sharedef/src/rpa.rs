use std::fmt;

/// Enum of RPA engings to watch.
#[derive(serde::Serialize, serde::Deserialize)]
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
}

impl fmt::Display for RpaEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.process_name())
    }
}

/// Collection of relevant data for the client to watch and send to the server.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(Default))]
pub struct RpaData {
    pub pid: u32,
    pub engine: RpaEngine,
    pub process: String,
    pub computer: String,
    pub env: Option<String>,
    pub run_id: String,
    pub azure_data: Option<AzureData>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(Default))]
pub struct AzureData {}

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
}
