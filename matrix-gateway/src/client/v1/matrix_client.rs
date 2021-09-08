use actix_web::client::Client;
use actix_web::http::Uri;
use super::{ClientConfig, ApiUriBuilder};
use super::model::{Flow, FlowCollection, LoginResponse, LoginRequest};
use actix_web::http;
use std::collections::HashMap;

pub struct MatrixClient
{
    api_uri: ApiUriBuilder,
    http_client: Client,
}

impl MatrixClient
{
    pub fn new(api_uri: ApiUriBuilder, http_client: Client) -> Self
    {
        Self {
            api_uri,
            http_client,
        }
    }

    pub async fn get_login(&self) -> Vec<Flow>
    {
        let mut response = self.http_client
            .get(&self.api_uri.login())
            .send()
            .await
            .unwrap();

        let bytes = response.body().await.unwrap();
        let flows: FlowCollection = serde_json::from_slice(&*bytes).unwrap();
        flows.flows
    }

    pub async fn post_login(&self, req: &LoginRequest) -> LoginResponse
    {
        let mut response = self.http_client
            .post(&self.api_uri.login())
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

    /*
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
    }*/
}