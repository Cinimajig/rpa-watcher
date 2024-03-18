use std::{env, fs};

const DEFAULT_PORT: u16 = 80;
const DEFAULT_API_VERSION: &str = "v9.2";


pub struct PRConfig {
    pub http_port: u16,
    pub db_conn_str: Option<String>,
}

impl PRConfig {
    pub fn load() -> Self {
        let http_port = match env::var("HTTP_PLATFORM_PORT").or(env::var("ASPNETCORE_PORT")) {
            Ok(port) => port.parse().unwrap_or(DEFAULT_PORT),
            _ => DEFAULT_PORT,
        };

        let mut this = Self {
            http_port,
            db_conn_str: None,
        };

        let Ok(mut db_config) = env::current_exe() else {
            return this;
        };
        db_config.pop();
        db_config.push("db.conn");

        if db_config.is_file() {
            if let Ok(db_config) = fs::read_to_string(db_config) {
                this.db_conn_str = parse_db_config(&db_config);
            }
        }

        this
    }
}

fn parse_db_config(config: &str) -> Option<String> {
    let text = config.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}
