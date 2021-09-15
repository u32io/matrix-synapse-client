use crate::constants::{AuthenticationType, IdentifierType};
use serde::Serialize;

#[derive(Serialize)]
pub struct LoginRequest {
    #[serde(rename = "type")]
    pub auth_type: AuthenticationType,
    pub identifier: LoginIdentifier,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginIdentifier {
    #[serde(rename = "type")]
    pub id_type: IdentifierType,
    pub user: String,
}
