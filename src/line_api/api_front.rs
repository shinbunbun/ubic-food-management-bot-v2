use awc::{self, ClientRequest};

use super::error::Error;
use super::error::Error::EnvError;

use super::authenticator::Authenticator;

#[derive(Debug)]
pub struct ApiFront<T>
where
    T: Authenticator,
{
    authenticator: T,
    base_url: String,
}

impl<T> ApiFront<T>
where
    T: Authenticator,
{
    pub fn new(authenticator: T, base_url: String) -> ApiFront<T> {
        ApiFront {
            authenticator,
            base_url,
        }
    }

    fn make_request_uri(&self, uri: String) -> String {
        format!("{}/{}", self.base_url, uri)
    }

    fn get_authorization_header(&self) -> Result<(&str, String), Error> {
        Ok((
            "Authorization",
            self.authenticator.get_authorization_token()?,
        ))
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
