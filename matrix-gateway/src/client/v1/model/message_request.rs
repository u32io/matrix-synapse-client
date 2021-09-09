use serde::{Serialize};
use super::MessageType;

#[derive(Serialize)]
pub struct MessageRequest
{
    #[serde(rename = "msgtype")]
    pub msg_type: MessageType,
    pub body: String,
}