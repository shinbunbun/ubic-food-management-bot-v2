use std::env::{self, VarError};

fn get_env(key: String) -> Result<String, VarError> {
    env::var(key)
}

pub fn get_token() -> Result<String, VarError> {
    get_env("CHANNEL_ACCESS_TOKEN".to_string())
}

pub fn get_secret() -> Result<String, VarError> {
    get_env("CHANNEL_SECRET".to_string())
}

pub fn get_id() -> Result<String, VarError> {
    get_env("CHANNEL_ID".to_string())
}

pub fn get_line_api_token() -> Result<String, VarError> {
    get_env("LINE_API_TOKEN".to_string())
}

pub fn get_api_base_url() -> Result<String, VarError> {
    get_env("API_BASE_URL".to_string())
}
