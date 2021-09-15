use crate::constants::MessageType;
use serde::Serialize;

#[derive(Serialize)]
pub struct MessageRequest {
    #[serde(rename = "msgtype")]
    pub msg_type: MessageType,
    pub body: String,
}
