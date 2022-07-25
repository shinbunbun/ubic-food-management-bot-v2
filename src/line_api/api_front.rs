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

#[cfg(test)]
mod tests {
    use crate::line_api::authenticator::Authenticator;

    use super::ApiFront;

    struct MockAuthenticator {}
    impl Authenticator for MockAuthenticator {
        fn get_authorization_token(&self) -> Result<String, crate::line_api::error::Error> {
            Ok("MockToken".to_string())
        }
    }

    #[test]
    fn hoge() {
        let auth = MockAuthenticator {};
        let front = ApiFront::new(auth, "https://mock-url".to_string());
        let request = front.get_request("hello".to_string()).unwrap();
        assert_eq!(request.get_uri().to_string(), "https://mock-url/hello");
    }
}
