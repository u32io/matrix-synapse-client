use crate::constants::{AuthenticationType, IdentifierType};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    #[serde(rename = "type")]
    pub auth_type: AuthenticationType,
    pub identifier: LoginIdentifier,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginIdentifier {
    #[serde(rename = "type")]
    pub id_type: IdentifierType,
    pub user: String,
}
