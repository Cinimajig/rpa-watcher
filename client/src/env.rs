#![allow(dead_code)]

use std::{env, fs, io, path::PathBuf, str::FromStr};

const DEFAULT_URL: &str = "http://localhost/api/checkin";
const DEFAULT_TOKEN: &str = "";
const DEFAULT_NOTIFICATION: NotificationType = NotificationType::None;

#[derive(Debug)]
pub struct Environment {
    pub url: String,
    pub token: String,
    pub notification: NotificationType,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            url: String::from(DEFAULT_URL),
            token: String::from(DEFAULT_TOKEN),
            notification: NotificationType::None,
        }
    }
}

impl Environment {
    pub fn from_file_then_env() -> Self {
        let mut this = Self::from_file().unwrap_or_default();

        match (env::var("RW_URL"), env::var("RW_TOKEN"), env::var("RW_NOTIFY")) {
            (Ok(url), Ok(token), Ok(notification)) => {
                this.url = url;
                this.token = token;
                this.notification = notification.parse().unwrap_or_default();
            }
            (Ok(url), Err(_), Err(_)) => {
                this.url = url;
            }
            (Err(_), Ok(token), Err(_)) => {
                this.token = token;
            }
            (Err(_), Err(_), Ok(notification)) => {
                this.notification = notification.parse().unwrap_or_default();
            }
            (Err(_), Err(_), Err(_)) => (),
            (Ok(url), Ok(token), Err(_)) => {
                this.url = url;
                this.token = token;
            },
            (Ok(url), Err(_), Ok(notification)) => {
                this.url = url;
                this.notification = notification.parse().unwrap_or_default();
            },
            (Err(_), Ok(token), Ok(notification)) => {
                this.token = token;
                this.notification = notification.parse().unwrap_or_default();
            },
        }

        this
    }

    pub fn from_env() -> Result<Self, env::VarError> {
        let url = env::var("RW_URL").unwrap_or(DEFAULT_URL.to_string());
        let token = env::var("RW_TOKEN").unwrap_or(DEFAULT_TOKEN.to_string());
        let notification = env::var("RW_NOTIFY").map(|s| s.parse().unwrap_or_default()).unwrap_or(DEFAULT_NOTIFICATION);

        Ok(Self { url, token, notification })
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
                Some((s, v)) if s.trim().eq_ignore_ascii_case("URL") => {
                    this.url = v.trim().to_string();
                }
                Some((s, _)) if s.trim().eq_ignore_ascii_case("URL") => {
                    this.url = DEFAULT_URL.to_string();
                }
                Some((s, v)) if s.trim().eq_ignore_ascii_case("TOKEN") => {
                    this.token = v.trim().to_string();
                }
                Some((s, _)) if s.trim().eq_ignore_ascii_case("TOKEN") => {
                    this.token = DEFAULT_TOKEN.to_string();
                }
                Some((s, v)) if s.trim().eq_ignore_ascii_case("NOTIFY") => {
                    this.notification = v.parse().unwrap_or_default();
                }
                Some((s, _)) if s.trim().eq_ignore_ascii_case("NOTIFY") => {
                    this.notification = DEFAULT_NOTIFICATION;
                }
                _ => (),
            };
        }

        Ok(this)
    }
}


#[derive(Debug)]
pub enum NotificationType {
    None,
    Window(Vec<String>),
    File(PathBuf)
}

impl Default for NotificationType {
    fn default() -> Self {
        Self::None
    }
}

impl FromStr for NotificationType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("Window(") && s.ends_with(")") {
            let start = s.find('(').expect("string is dropped?") + 1;
            let end = s.find(')').expect("string is dropped?");
            let value = &s[start..end];

            let mut classes = Vec::with_capacity(2);
            for class in value.split("->") {
                classes.push(class.trim().to_string());
            }

            Ok(Self::Window(classes))

        } else if s.starts_with("File(") && s.ends_with(")") {
            let start = s.find('(').expect("string is dropped?") + 1;
            let end = s.find(')').expect("string is dropped?");
            let value = &s[start..end];

            Ok(NotificationType::File(PathBuf::from(value)))
            
        } else {
            Err("no valid value found. It should be either Window(<class_name> -> <class_name?>) or File(<file_path>)")
        }
    }
}