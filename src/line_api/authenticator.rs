use super::error::Error;
use crate::config;

pub trait Authenticator {
    fn get_authorization_token(&self) -> Result<String, Error>;
}

#[derive(Debug)]
pub struct BearerAuthenticator {}

impl BearerAuthenticator {
    pub fn new() -> BearerAuthenticator {
        BearerAuthenticator {}
    }
}

impl Authenticator for BearerAuthenticator {
    fn get_authorization_token(&self) -> Result<String, Error> {
        let token = config::get_line_api_token().map_err(|e| Error::EnvError(e))?;
        Ok(format!("Bearer {}", token))
    }
}
