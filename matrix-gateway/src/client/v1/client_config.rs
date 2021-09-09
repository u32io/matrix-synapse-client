use std::convert::TryFrom;
use actix_web::http::Uri;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Serialize)]
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

impl TryFrom<&std::path::Path> for ClientConfig
{
    type Error = serde_json::Error;

    fn try_from(path: &std::path::Path) -> Result<Self, Self::Error> {
        let content = std::fs::read_to_string(path)
            .unwrap_or_else(|e|{
                panic!("Invalid file: {:?}", path);
            });
        serde_json::from_str(content.as_str())
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn config_deserializes_from_json() -> ()
    {
        let json = "
            {
                \"home_server\":\"homeserver\",
                \"authority\":\"authority\",
                \"client_api\":\"/_matrix/client/r0\"
            }";

        let config = ClientConfig::try_from(json);
        assert!(config.is_ok());
    }
}