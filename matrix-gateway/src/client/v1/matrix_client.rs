use actix_web::client::{Client, SendRequestError, PayloadError, ClientResponse};
use actix_web::http::{Uri, Error, StatusCode};
use super::{ClientConfig, ApiUriBuilder};
use super::model::{Flow, FlowCollection, LoginResponse, LoginRequest, ErrorResponse};
use std::collections::HashMap;
use actix_web::ResponseError;
use serde::de::DeserializeOwned;
use actix_web::web::Payload;
use actix_web::dev::PayloadStream;

// TODO: implement std::error::Error on this type
pub enum MatrixClientError
{
    SendRequestError(SendRequestError),
    PayloadErr(PayloadError),
    JsonDeserializationError(serde_json::Error),
    HttpResponseError(StatusCode, ErrorResponse),
    Unknown,
}

pub struct MatrixClient
{
    api_uri: ApiUriBuilder,
    http_client: Client,
}

macro_rules! http_get {
    ($http_client:expr, $uri:expr) => {
        $http_client
        .get($uri)
        .send()
        .await
        .map_err(|e| MatrixClientError::SendRequestError(e))
    }
}

macro_rules! http_post {
    ($http_client:expr, $uri:expr, $json:expr) => {
        $http_client
        .post($uri)
        .send_json($json)
        .await
        .map_err(|e| MatrixClientError::SendRequestError(e))
    }
}

macro_rules! get_json {
    ($t:ty, $response:expr) => {
        {
            let bytes = $response.body()
                .await
                .map_err(|e| MatrixClientError::PayloadErr(e))?;

            let json: Result<$t,MatrixClientError> = serde_json::from_slice(&*bytes)
                .map_err(|e| MatrixClientError::JsonDeserializationError(e));
            json
        }
    }
}

macro_rules! try_convert_200 {
    ($http_response:expr, $model:ty) => {
        match $http_response.status()
        {
            StatusCode::OK => Ok(get_json!($model, $http_response)?),
            _ => Err(MatrixClientError::HttpResponseError($http_response.status()
                , get_json!(ErrorResponse, $http_response)?))
        }
    }
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

    pub async fn get_login(&self) -> Result<FlowCollection, MatrixClientError>
    {
        let mut response = http_get!(self.http_client, self.api_uri.login())?;
        try_convert_200!(response, FlowCollection)
    }

    pub async fn post_login(&self, req: &LoginRequest) -> Result<LoginResponse, MatrixClientError>
    {
        let mut response = http_post!(self.http_client, self.api_uri.login(), req)?;
        try_convert_200!(response, LoginResponse)
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