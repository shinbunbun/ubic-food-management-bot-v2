use awc;
use serde::de::DeserializeOwned;

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

    async fn get<R: DeserializeOwned>(&self, uri: &str) -> Result<R, Error> {
        let req = awc::Client::default();
        let token = self
            .authenticator
            .get_authorization_token()
            .map_err(|e| EnvError(e))?;
        println!("{}", uri);
        println!("{}", token);
        let req = req
            .get(uri)
            .insert_header(("Authorization", format!("Bearer {}", &token)));
        let mut response = req.send().await.map_err(|e| ApiError(e.to_string()))?;
        if response.status() != 200 {
            println!("{}", response.status());
            return Err(ApiError(
                format!("Http status was {}", response.status()).to_string(),
            ));
        }
        let body = response
            .body()
            .await
            .map_err(|e| ApiError(e.to_string()))?
            .to_vec();
        serde_json::from_slice(&body).map_err(|e| SerdeError(e.to_string()))
    }
}
