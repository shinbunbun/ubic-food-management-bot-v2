use crate::config;

#[derive(Debug)]
pub struct Authenticator {}

impl Authenticator {
    pub fn get_authorization_token(&self) -> Result<String, std::env::VarError> {
        config::get_line_api_token()
    }
    pub fn new() -> Authenticator {
        Authenticator {}
    }
}
