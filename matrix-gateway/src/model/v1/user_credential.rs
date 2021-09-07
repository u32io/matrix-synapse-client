use std::convert::TryFrom;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserCredential
{
    pub user_name: String,
    pub password: String,
}

impl TryFrom<&str> for UserCredential
{
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}