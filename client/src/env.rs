#![allow(dead_code)]

use std::{io, env, fs};

const DEFAULT_URL: &str = "http://localhost/api/checkin";
const DEFAULT_TOKEN: &str = "";

#[derive(Debug)]
pub struct Environment {
    pub url: String,
    pub token: String,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            url: String::from(DEFAULT_URL),
            token: String::from(DEFAULT_TOKEN),
        }
    }
}

impl Environment {
    pub fn from_file_then_env() -> Self {
        let mut this = Self::from_file().unwrap_or_default();

        match (env::var("RW_URL"), env::var("RW_TOKEN")) {
            (Ok(url), Ok(token)) => { this.url = url; this.token = token; },
            (Ok(url), Err(_)) => { this.url = url; },
            (Err(_), Ok(token)) => { this.token = token; },
            (Err(_), Err(_)) => (),
        }

        this
    }


    pub fn from_env() -> Result<Self, env::VarError> {
        let url = env::var("RW_URL")?;
        let token = env::var("RW_TOKEN")?;

        Ok(Self { url, token })
    }

    pub fn from_file() -> io::Result<Self> {
        // Reads settings.ini.
        let mut path = env::current_exe()?;
        path.pop();
        path.push("settings.ini");
        let file = fs::read_to_string(path)?;

        // Variable for our values.
        let mut this = Self::default();

        // Loops through the file
        for line in file.lines() {
            // If the linek starts with ;, then it's a comment.
            if line.trim_start().starts_with(';') {
                continue;
            }

            // Finds the variables we care about.
            match line.split_once('=') {
                Some((s, v)) if s.trim().eq_ignore_ascii_case("URL") => this.url = v.trim().to_string(),
                Some((s, _)) if s.trim().eq_ignore_ascii_case("URL") => this.url = DEFAULT_URL.to_string(),
                Some((s, v)) if s.trim().eq_ignore_ascii_case("TOKEN") => this.token = v.trim().to_string(),
                Some((s, _)) if s.trim().eq_ignore_ascii_case("TOKEN") => this.token = DEFAULT_TOKEN.to_string(),
                _ => (),
            };
        }

        Ok(this)
    }
}