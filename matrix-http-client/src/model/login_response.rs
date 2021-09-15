use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub user_id: String,
    pub access_token: String,
    pub home_server: String,
    pub device_id: String,
}
