use actix_web::client::Client;
use actix_web::http::StatusCode;
use std::convert::TryFrom;
use std::path::Path;
use matrix_http_client::{MatrixClient, ApiUriBuilder, ClientConfig, TMatrixClient};
use matrix_http_client::model::{LoginRequest, LoginIdentifier};
use matrix_http_client::constants::{AuthenticationType, IdentifierType};
use matrix_http_client::abstraction::GetError;

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

    let req = LoginRequest {
        auth_type: AuthenticationType::Dummy,
        identifier: LoginIdentifier {
            id_type: IdentifierType::ThirdParty,
            user: "non_user".to_string(),
        },
        password: "password123".to_string(),
    };

    let resp = matrix.post_login(&req).await;

    assert!(resp.is_err());
    let err = resp.unwrap_err();
    let err = err.get_error().unwrap();

    assert_eq!(StatusCode::BAD_REQUEST, err.status());
}

#[actix_rt::test]
async fn matrix_client_returns_403_when_credentials_are_invalid(){
    let matrix = init_matrix_client();

    let req = LoginRequest {
        auth_type: AuthenticationType::Password,
        identifier: LoginIdentifier {
            id_type: IdentifierType::User,
            user: "non_user".to_string(),
        },
        password: "password123".to_string(),
    };

    let resp = matrix.post_login(&req).await;

    assert!(resp.is_err());
    let err = resp.unwrap_err();
    let err = err.get_error().unwrap();

    assert_eq!(StatusCode::FORBIDDEN, err.status());
}