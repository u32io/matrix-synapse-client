use std::convert::TryFrom;
use crate::web::AppState;
use actix_web::client::Client;
use actix_web::http::Uri;
use actix_web::http::uri::Scheme;
use crate::model::ChatRoomMessage;
use super::text_message::{TextMessage, TextMessageContent, Unsigned};
use super::ClientConfig;
use super::model::{Flow, FlowCollection, LoginResponse, LoginRequest};
use actix_web::http;
use std::collections::HashMap;

pub struct MatrixClient
{
    internal: ClientConfig,
    http_client: Client,
    api: Api,
}

struct Api
{
    login: Uri,
}

impl TryFrom<&ClientConfig> for Api
{
    type Error = http::Error;

    fn try_from(config: &ClientConfig) -> Result<Self, Self::Error> {
        let uri_builder = Uri::builder()
            .scheme(Scheme::HTTPS)
            .authority(config.authority.as_str());

        Ok(Self {
            login: uri_builder
                .path_and_query(format!("{0}/login", config.client_api.as_str()))
                .build()?
        })
    }
}

impl MatrixClient
{
    pub fn new(config: ClientConfig, client: Client) -> Result<Self,http::Error>
    {
        Ok(Self {
            http_client: client,
            api: Api::try_from(&config)?,
            internal: config,
        })
    }

    pub async fn get_login(&self) -> Vec<Flow>
    {
        let mut response = self.http_client.get(&self.api.login)
            .send()
            .await
            .unwrap();

        let bytes = response.body().await.unwrap();
        let flows: FlowCollection = serde_json::from_slice(&*bytes).unwrap();
        flows.flows
    }

    pub async fn post_login(&self, req: &LoginRequest) -> LoginResponse
    {
        let mut response = self.http_client.post(&self.api.login)
            .send_json(req)
            .await
            .unwrap();

        let bytes = response.body().await.unwrap();
        serde_json::from_slice(&*bytes).unwrap()
    }
}

#[cfg(test)]
mod test
{
    use std::fs;
    use super::*;
    use crate::model::UserCredential;
    use crate::client::v1::model::{AuthenticationType, LoginIdentifier, IdentifierType};

    #[actix_rt::test]
    async fn foo() -> ()
    {
        let config = fs::read_to_string(".client.json").unwrap();
        let config = ClientConfig::try_from(config.as_str()).unwrap();

        let client = Client::default();

        let matrix = MatrixClient::new(config, client).unwrap();
        let x = matrix.get_login().await;

        println!("{:?}", x);
    }

    #[actix_rt::test]
    async fn foo2() -> ()
    {
        let config = fs::read_to_string(".client.json").unwrap();
        let config = ClientConfig::try_from(config.as_str()).unwrap();

        let client = Client::default();

        let matrix = MatrixClient::new(config, client).unwrap();

        let users = fs::read_to_string(".users.json").unwrap();
        let users: Vec<UserCredential> = serde_json::from_str(users.as_str()).unwrap();

        let user = users.get(0).unwrap();

        let req = LoginRequest {
            auth_type: AuthenticationType::Password,
            identifier: LoginIdentifier {
                id_type: IdentifierType::User,
                user: user.user_name.clone(),
            },
            password: user.password.clone(),
        };

        let res = matrix.post_login(&req).await;

        println!("{:?}", res);
    }
}