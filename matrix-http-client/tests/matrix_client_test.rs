use actix_web::client::Client;
use actix_web::http::StatusCode;
use std::convert::TryFrom;
use std::path::Path;
use matrix_http_client::{MatrixClient, ApiUriBuilder, ClientConfig, TMatrixClient};
use matrix_http_client::abstraction::GetError;
use urlencoding::Encoded;

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

#[actix_rt::test]
async fn matrix_client_returns_400_when_flow_is_invalid(){
    let matrix = init_matrix_client();

    let req = matrix_web_util::fs::read_file_as_unchecked(
        "test_resource/model/login_request/invalid_flow.json");

    let resp = matrix.post_login(&req).await;

    assert!(resp.is_err());
    let err = resp.unwrap_err();
    let err = err.get_error().unwrap();

    assert_eq!(StatusCode::BAD_REQUEST, err.status());
}

#[actix_rt::test]
async fn matrix_client_returns_403_when_credentials_are_invalid(){
    let matrix = init_matrix_client();

    let req = matrix_web_util::fs::read_file_as_unchecked(
        "test_resource/model/login_request/invalid_credential.json");

    let resp = matrix.post_login(&req).await;

    assert!(resp.is_err());
    let err = resp.unwrap_err();
    let err = err.get_error().unwrap();

    assert_eq!(StatusCode::FORBIDDEN, err.status());
}

#[actix_rt::test]
async fn matrix_client_returns_200_after_valid_login_and_message_send(){
    let matrix = init_matrix_client();
    // To perform this test, we'll need a LoginRequest, a room id, and a message.
    // Get a valid login request
    let req = matrix_web_util::fs::read_file_as_unchecked(
        "test_resource/model/login_request/.valid_credential.json");
    // Get a valid room id
    let room_id: String = matrix_web_util::fs::read_file_as_unchecked(
        "test_resource/misc/.room_id.json");
    // Get a valid message
    let msg = matrix_web_util::fs::read_file_as_unchecked(
        "test_resource/model/message_request/hello.json");
    // Login
    let resp = matrix.post_login(&req).await;
    // Assert we have logged in successfully in order to proceed
    assert!(resp.is_ok());
    // Extract the LoginResponse, where we will have our access token
    let login_resp = resp.unwrap();
    // Send a message request using the above Message, room id, and the access token from the
    // LoginResponse
    let msg_req = matrix.post_message(
        &msg,
        Encoded::new(room_id.as_str()),
        login_resp.access_token.as_str())
        .await;

    assert!(msg_req.is_ok());
}