use std::convert::TryFrom;
use actix_web::http::Uri;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ClientConfig
{
    pub home_server: String,
    pub authority: String, // TODO: impl Deserialize for Uri
    pub client_api: String,
}

impl TryFrom<&str> for ClientConfig
{
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}