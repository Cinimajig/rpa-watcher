use serde::{Deserialize, Serialize};
use std::{env, fs, io};
use tokio::{sync::RwLock, time::Instant};

const DEFAULT_VERSION: &str = "9.2";

// static mut CONNECTION: Option<RwLock<PowerAutomateAPI>> = None;

#[derive(Debug, Serialize)]
pub struct OAuthData {
    org_env: (String, String),
    client_id: String,
    client_secret: String,
    grant_type: String,
}

impl Default for OAuthData {
    fn default() -> Self {
        Self {
            org_env: Default::default(),
            client_id: Default::default(),
            client_secret: Default::default(),
            grant_type: "client_credentials".to_string(),
        }
    }
}

pub struct PowerAutomateAPI {
    version: String,
    cred: OAuthData,
    teanant_id: String,
    token: Token,
    token_time: Instant,
}

impl PowerAutomateAPI {
    pub fn load() -> io::Result<Self> {
        let mut this = Self {
            cred: OAuthData::default(),
            version: DEFAULT_VERSION.to_string(),
            teanant_id: String::new(),
            token: Token {
                token_type: Default::default(),
                expires_in: Default::default(),
                ext_expires_in: Default::default(),
                access_token: Default::default(),
            },
            token_time: Instant::now(),
        };

        let Ok(mut pa_config) = env::current_exe() else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "path to the server wasn't found",
            ));
        };
        pa_config.pop();
        pa_config.push("flow.conn");

        if pa_config.is_file() {
            if let Ok(pa_config) = fs::read_to_string(pa_config) {
                fill_oauth(&mut this, &pa_config)?;
            }
        }

        Ok(this)
    }

    pub fn is_empty(&self) -> bool {
        self.version.is_empty()
            && self.cred.client_id.is_empty()
            && self.cred.client_secret.is_empty()
            && self.cred.grant_type.is_empty()
            && self.cred.org_env.0.is_empty()
            && self.cred.org_env.1.is_empty()
            && self.teanant_id.is_empty()
    }

    pub async fn authenticate(&mut self) -> reqwest::Result<()> {
        self.token
            .authenticate(&self.cred, &self.teanant_id)
            .await?;
        self.token_time = Instant::now();
        Ok(())
    }

    pub fn is_authenticated(&self) -> bool {
        !self.token.access_token.is_empty()
            && self.token_time.elapsed().as_secs() < self.token.expires_in
    }
}

fn fill_oauth(oauth: &mut PowerAutomateAPI, content: &str) -> io::Result<()> {
    for line in content.lines() {
        match line.split_once('=') {
            Some(("ClientId", val)) => oauth.cred.client_id = val.to_string(),
            Some(("ClientSecret", val)) => oauth.cred.client_secret = val.to_string(),
            Some(("OrgId", val)) => {
                if let Some((part1, part2)) = val.split_once('.') {
                    oauth.cred.org_env.0 = part1.to_string();
                    oauth.cred.org_env.1 = part2.to_string();
                }
            }
            Some(("TeanantId", val)) => oauth.teanant_id = val.to_string(),
            Some((_, _)) => (),
            None => (),
        }
    }

    if oauth.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "not all fields were filled",
        ))
    } else {
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct Token {
    token_type: String,
    expires_in: u64,
    ext_expires_in: u64,
    access_token: String,
}

impl Token {
    pub async fn authenticate(
        &mut self,
        cred: &OAuthData,
        teanant_id: &str,
    ) -> reqwest::Result<()> {
        let res = reqwest::Client::new()
            .post(format!(
                "https://login.microsoftonline.com/{teanant_id}/oauth2/v2.0/token"
            ))
            .form(cred)
            .send()
            .await?
            .error_for_status()?;

        *self = res.json().await?;
        Ok(())
    }
}

pub async fn lookup_uiflow(paapi: &mut PowerAutomateAPI, flow_id: &str) -> anyhow::Result<String> {
    if !paapi.is_authenticated() {
        paapi.authenticate().await?;
        // match paapi.authenticate().await {
        //     Ok(_) => (),
        //     Err(err) => return Err(Error::new(ErrorKind::ConnectionRefused, format!("{}", err.without_url()))),
        // };
    }

    // 6 = Desktop flow
    let res = reqwest::get(format!("https://{org1}.api.{org2}.dynamics.com/api/data/v{api_version}/workflows({flow_id})?$select=name&$filter=category+eq+6", org1=&paapi.cred.org_env.0, org2=&paapi.cred.org_env.1, api_version=paapi.version)).await?.error_for_status()?;
    let json: serde_json::Value = res.json().await?;

    match json.get("name") {
        Some(name) => Ok(name.to_string()),
        None => Err(anyhow::anyhow!("failed to find name of flow id: {flow_id}")),
    }
}
