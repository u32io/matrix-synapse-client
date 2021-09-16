use super::constants::RoomEventType;
use super::model::{
    ErrorResponse, EventResponse, FlowCollection, LoginRequest, LoginResponse, MessageRequest,
};
use super::ApiUriBuilder;
use actix_web::client::{Client};
use actix_web::http::StatusCode;
use urlencoding::Encoded;
use std::future::Future;
use std::pin::Pin;
use crate::TMatrixClient;
use crate::error::{MatrixClientError, HttpResponseError};

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
            _ => Err(MatrixClientError::HttpResponseError(HttpResponseError {
                status: $http_response.status(),
                body: get_json!(ErrorResponse, $http_response)?,
            })),
        }
    };
}

pub struct MatrixClient {
    internal: InternalMatrixClient,
}

impl MatrixClient {
    pub fn new(api_uri: ApiUriBuilder, http_client: Client) -> Self {
        Self {
            internal: InternalMatrixClient {
                api_uri,
                http_client,
            }
        }
    }
}

impl TMatrixClient for MatrixClient {
    /// `GET` the authentication scheme of the matrix-synapse API
    fn get_login<'req>(&'req self) -> Pin<Box<dyn Future<Output=Result<FlowCollection,MatrixClientError>> + 'req>> {
        Box::pin(self.internal.get_login())
    }
    /// `POST` the credentials of a user and expect a `200` response with an access token
    fn post_login<'req>(&'req self, req: &'req LoginRequest) -> Pin<Box<dyn Future<Output=Result<LoginResponse, MatrixClientError>> + 'req>> {
        Box::pin(self.internal.post_login(req))
    }
    /// `POST` a basic message and expect and expect a response that contains an event id
    /// ```bash
    /// curl -XPOST -d '{"msgtype":"m.text", "body":"hello"}' \
    ///     "https://API/send/m.room.message?access_token=YOUR_ACCESS_TOKEN"
    ///
    /// { "event_id": "EVENT ID" }
    /// ```
    fn post_message<'req>(
        &'req self,
        msg: &'req MessageRequest,
        room_id: Encoded<&'req str>,
        access_token: &'req str
    ) -> Pin<Box<dyn Future<Output=Result<EventResponse, MatrixClientError>> + 'req>> {
        Box::pin(self.internal.post_message(msg, room_id, access_token))
    }
}

struct InternalMatrixClient {
    api_uri: ApiUriBuilder,
    http_client: Client,
}

impl InternalMatrixClient {
    /// `GET` the authentication scheme of the matrix-synapse API
    async fn get_login(&self) -> Result<FlowCollection, MatrixClientError> {
        let mut response = http_get!(self.http_client, self.api_uri.login())?;
        try_convert_200!(response, FlowCollection)
    }
    /// `POST` the credentials of a user and expect a `200` response with an access token
    async fn post_login(&self, req: &LoginRequest) -> Result<LoginResponse, MatrixClientError> {
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
    async fn post_message(
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