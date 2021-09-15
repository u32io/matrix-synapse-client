use super::constants::RoomEventType;
use super::model::{
    ErrorResponse, EventResponse, FlowCollection, LoginRequest, LoginResponse, MessageRequest,
};
use super::ApiUriBuilder;
use actix_web::client::{Client, PayloadError, SendRequestError};
use actix_web::http::StatusCode;
use urlencoding::Encoded;

// TODO: implement std::error::Error on this type
pub enum MatrixClientError {
    SendRequestError(SendRequestError),
    PayloadErr(PayloadError),
    JsonDeserializationError(serde_json::Error),
    HttpResponseError(StatusCode, ErrorResponse),
    Unknown,
}

pub struct MatrixClient {
    api_uri: ApiUriBuilder,
    http_client: Client,
}
/// A template for building `GET` requests and mapping their `Err` to `MatrixClientErr`
macro_rules! http_get {
    ($http_client:expr, $uri:expr) => {
        $http_client
            .get($uri)
            .send()
            .await
            .map_err(|e| MatrixClientError::SendRequestError(e))
    };
}
/// A template for building `POST` requests and mapping their `Err` to `MatrixClientErr`.
/// This macro expects a body that serializes to json.
macro_rules! http_post {
    ($http_client:expr, $uri:expr, $json:expr) => {
        $http_client
            .post($uri)
            .send_json($json)
            .await
            .map_err(|e| MatrixClientError::SendRequestError(e))
    };
}
/// A template for extracting json from `HttpResponse` and mapping both the `PayloadErr` and
/// `JsonDeserializationError` to `MatrixClientErr`
macro_rules! get_json {
    ($t:ty, $response:expr) => {{
        let bytes = $response
            .body()
            .await
            .map_err(|e| MatrixClientError::PayloadErr(e))?;

        let json: Result<$t, MatrixClientError> = serde_json::from_slice(&*bytes)
            .map_err(|e| MatrixClientError::JsonDeserializationError(e));
        json
    }};
}
/// I don't even know how to explain this one. It basically does everything.
macro_rules! try_convert_200 {
    ($http_response:expr, $model:ty) => {
        match $http_response.status() {
            StatusCode::OK => Ok(get_json!($model, $http_response)?),
            _ => Err(MatrixClientError::HttpResponseError(
                $http_response.status(),
                get_json!(ErrorResponse, $http_response)?,
            )),
        }
    };
}

impl MatrixClient {
    pub fn new(api_uri: ApiUriBuilder, http_client: Client) -> Self {
        Self {
            api_uri,
            http_client,
        }
    }

    /// `GET` the authentication scheme of the matrix-synapse API
    pub async fn get_login(&self) -> Result<FlowCollection, MatrixClientError> {
        let mut response = http_get!(self.http_client, self.api_uri.login())?;
        try_convert_200!(response, FlowCollection)
    }

    /// `POST` the credentials of a user and expect a `200` response with an access token
    pub async fn post_login(&self, req: &LoginRequest) -> Result<LoginResponse, MatrixClientError> {
        let mut response = http_post!(self.http_client, self.api_uri.login(), req)?;
        try_convert_200!(response, LoginResponse)
    }

    /// `POST` a basic message and expect and expect a response that contains an event id
    /// ```bash
    /// curl -XPOST -d '{"msgtype":"m.text", "body":"hello"}' \
    ///     "https://API/send/m.room.message?access_token=YOUR_ACCESS_TOKEN"
    ///
    /// { "event_id": "EVENT ID" }
    /// ```
    pub async fn post_message(
        &self,
        msg: &MessageRequest,
        room_id: Encoded<&str>,
        access_token: &str,
    ) -> Result<EventResponse, MatrixClientError> {
        let mut response = http_post!(
            self.http_client,
            self.api_uri
                .send(room_id, RoomEventType::Message, access_token),
            msg
        )?;
        try_convert_200!(response, EventResponse)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::client::Client;
    use std::convert::TryFrom;
    use std::path::Path;
    use crate::ClientConfig;

    fn init_matrix_client() -> MatrixClient {
        let config = ClientConfig::try_from(Path::new(".client.json")).unwrap();
        let api_uri_builder =
            ApiUriBuilder::new(config.authority.as_str(), config.client_api.as_str()).unwrap();
        let actix_http_client = Client::default();

        MatrixClient::new(api_uri_builder, actix_http_client)
    }

    #[actix_rt::test]
    async fn matrix_client_returns_flow_on_successful_get_login() {
        let matrix = init_matrix_client();

        let flow = matrix.get_login().await;

        assert!(flow.is_ok());

        let flow = flow.ok().unwrap().flows;
        assert!(flow.len() >= 1);
    }
}
