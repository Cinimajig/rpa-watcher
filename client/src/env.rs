use std::{io, env};

pub struct Environment {
    pub url: String,
    pub token: String,
}

impl Environment {
    pub fn from_env() -> Result<Self, env::VarError> {
        let url = env::var("RW_URL")?;
        let token = env::var("RW_URL")?;

        Ok(Self { url, token, })
    }

    pub fn from_file() -> io::Result<Self> {


        Err(io::Error::last_os_error())
    }
}