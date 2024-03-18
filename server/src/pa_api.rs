use std::{env, fs, io};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

const DEFAULT_VERSION: &str = "9.2";

static mut CONNECTION: Option<RwLock<PowerAutomateAPI>> = None;

#[derive(Debug, Serialize)]
pub struct OAuthData {
    org_env: String,
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
    token: Option<Token>,
}

impl PowerAutomateAPI {
    pub fn load() -> io::Result<Self> {
        let mut this = Self {
            cred: OAuthData::default(),
            version: DEFAULT_VERSION.to_string(),
            teanant_id: String::new(),
            token: None,
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
            && self.cred.org_env.is_empty()
            && self.teanant_id.is_empty()
    }
}

fn fill_oauth(oauth: &mut PowerAutomateAPI, content: &str) -> io::Result<()> {
    for line in content.lines() {
        match line.split_once('=') {
            Some(("ClientId", val)) => oauth.cred.client_id = val.to_string(),
            Some(("ClientSecret", val)) => oauth.cred.client_secret = val.to_string(),
            Some(("OrgId", val)) => oauth.cred.org_env = val.to_string(),
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
            .post(
                format!("https://login.microsoftonline.com/{teanant_id}/oauth2/v2.0/token")
            )
            .form(cred)
            .send()
            .await?
            .error_for_status()?;

        *self = res.json().await?;
        Ok(())
    }
}

pub async fn lookup_uiflow(flow_id: &str) -> tokio::io::Result<String> {
    use tokio::io::*;

    unsafe {
        match &mut CONNECTION {
            Some(conn) => {
                let mut writter = conn.write().await;
                

                todo!()
            },
            None => Err(Error::new(ErrorKind::PermissionDenied, "no token exists")),
        }
    }
    
}