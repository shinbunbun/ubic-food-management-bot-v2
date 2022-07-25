use awc::{self, ClientRequest, ClientResponse, SendClientRequest};
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::error::Error;
use super::error::Error::{ApiError, EnvError, SerdeError};

use super::authenticator::Authenticator;

#[derive(Debug)]
pub struct ApiFront {
    authenticator: Authenticator,
    base_url: String,
}

impl ApiFront {
    pub fn new(authenticator: Authenticator, base_url: String) -> ApiFront {
        ApiFront {
            authenticator,
            base_url,
        }
    }

    fn get_bearer_token(&self) -> Result<String, Error> {
        let token = self
            .authenticator
            .get_authorization_token()
            .map_err(|e| EnvError(e))?;
        let res = format!("Bearer {}", &token).to_string();
        Ok(res)
    }

    fn make_request_uri(&self, uri: String) -> String {
        format!("{}/{}", self.base_url, uri)
    }

    fn get_authorization_header(&self) -> Result<(&str, String), Error> {
        Ok(("Authorization", self.get_bearer_token()?))
    }

    pub fn get_request(&self, uri: String) -> Result<ClientRequest, Error> {
        let req = awc::Client::default()
            .get(self.make_request_uri(uri))
            .insert_header(self.get_authorization_header()?);
        Ok(req)
    }

    pub fn post_request(&self, uri: String) -> Result<ClientRequest, Error> {
        let req = awc::Client::default()
            .post(self.make_request_uri(uri))
            .insert_header(self.get_authorization_header()?);
        Ok(req)
    }

    pub fn delete_request(&self, uri: String) -> Result<ClientRequest, Error> {
        let req = awc::Client::default()
            .delete(self.make_request_uri(uri))
            .insert_header(self.get_authorization_header()?);
        Ok(req)
    }

    pub fn patch_request(&self, uri: String) -> Result<ClientRequest, Error> {
        let req = awc::Client::default()
            .patch(self.make_request_uri(uri))
            .insert_header(self.get_authorization_header()?);
        Ok(req)
    }
}
