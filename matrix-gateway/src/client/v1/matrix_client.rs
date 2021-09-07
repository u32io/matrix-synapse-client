use std::convert::TryFrom;
use crate::web::AppState;
use actix_web::client::Client;
use actix_web::http::Uri;
use actix_web::http::uri::Scheme;
use crate::model::ChatRoomMessage;
use super::text_message::{TextMessage, TextMessageContent, Unsigned};
use super::ClientConfig;
use super::model::{Flow, FlowCollection};
use actix_web::http;
use std::collections::HashMap;

pub struct MatrixClient
{
    internal: ClientConfig,
    http_client: Client,
}

impl MatrixClient
{
    pub fn new(config: ClientConfig, client: Client) -> Self
    {
        Self {
            internal: config,
            http_client: client
        }
    }

    pub async fn get_login(&self) -> Vec<Flow>
    {
        let uri = Uri::builder()
            .scheme(Scheme::HTTPS)
            .authority(self.internal.authority.as_str())
            .path_and_query(format!("{0}/login", &self.internal.client_api))
            .build()
            .unwrap();

        let mut response = self.http_client.get(uri)
            .send()
            .await
            .unwrap();

        let bytes = response.body().await.unwrap();
        let flows: FlowCollection = serde_json::from_slice(&*bytes).unwrap();
        flows.flows
    }
}

#[cfg(test)]
mod test
{
    use std::fs;
    use super::*;

    #[actix_rt::test]
    async fn foo() -> ()
    {
        let config = fs::read_to_string(".client.json").unwrap();
        let config = ClientConfig::try_from(config.as_str()).unwrap();

        let client = Client::default();

        let matrix = MatrixClient::new(config, client);
        let x = matrix.get_login().await;

        println!("{:?}", x);
    }
}