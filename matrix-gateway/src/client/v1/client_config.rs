use std::convert::TryFrom;
use actix_web::http::Uri;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientConfig
{
    pub home_server: String,
    pub authority: String, // TODO: impl Deserialize for Uri
    pub client_api: String,
    pub client_api_paths: HashMap<String,String>,
}

impl TryFrom<&str> for ClientConfig
{
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
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
                \"client_api\":\"/_matrix/client/r0\",
                \"client_api_paths\":{
                    \"GET login\":\"/_matrix/client/r0/login\"
                }
            }";

        let config = ClientConfig::try_from(json);
        assert!(config.is_ok());
    }
}