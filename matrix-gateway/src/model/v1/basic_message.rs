use serde::{Deserialize};

#[derive(Deserialize)]
pub struct BasicMessage
{
    pub room: String,
    pub user: String,
    pub body: String,
}