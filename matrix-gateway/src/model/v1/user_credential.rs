use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserCredential
{
    pub user_name: String,
    pub password: String,
}