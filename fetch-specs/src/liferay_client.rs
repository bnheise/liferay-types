use reqwest::{Client, IntoUrl, RequestBuilder};
use std::env;
use thiserror::Error;

const LIFERAY_USER_KEY: &str = "LIFERAY_USER";
const LIFERAY_PASS_KEY: &str = "LIFERAY_PASS";

pub struct LiferayClient {
    client: Client,
    username: String,
    password: String,
}

impl LiferayClient {
    pub fn new(username: String, password: String) -> Self {
        Self {
            client: Client::new(),
            username,
            password,
        }
    }

    pub fn from_env() -> Result<Self, LiferayClientError> {
        let username = env::var(LIFERAY_USER_KEY)
            .map_err(|_| LiferayClientError::Environment(LIFERAY_USER_KEY.into()))?;

        let password = env::var(LIFERAY_PASS_KEY)
            .map_err(|_| LiferayClientError::Environment(LIFERAY_PASS_KEY.into()))?;

        Ok(Self::new(username, password))
    }

    pub fn get<U>(&self, url: U) -> RequestBuilder
    where
        U: IntoUrl,
    {
        self.client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
    }
}

#[derive(Error, Debug)]
pub enum LiferayClientError {
    #[error("Failed to read environment variable {0}")]
    Environment(String),
}
