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
