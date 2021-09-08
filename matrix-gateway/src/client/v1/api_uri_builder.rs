use super::model::RoomEventType;
use std::str::FromStr;
use actix_web::http::{Error, Uri};
use actix_web::http::uri::{Scheme};

pub struct ApiUriBuilder
{
    base_uri: String,
}

impl ApiUriBuilder
{
    pub fn new(authority: &str, api_prefix: &str) -> Result<Self, Error>
    {
        let base_uri: String = Uri::builder()
            .scheme(Scheme::HTTPS)
            .authority(authority)
            .path_and_query(api_prefix)
            .build()?
            .to_string();
        Ok(ApiUriBuilder {
            base_uri
        })
    }

    pub fn login(&self) -> Uri
    {
        let uri = format!("{}/login", self.base_uri.as_str());
        Uri::from_str(uri.as_str()).unwrap()
    }

    // "/_matrix/client/r0/rooms/{ROOM_ID}/send/{ROOM_EVENT}?access_token={ACCESS_TOKEN}"
    pub fn send(&self, room_id: &str, room_event: RoomEventType, access_token: &str) -> Uri
    {
        let uri = format!("{}/rooms/{}/send/{}?access_token={}"
            , self.base_uri.as_str()
            , room_id
            , room_event.as_str()
            , access_token);
        Uri::from_str(uri.as_str()).unwrap()
    }
}