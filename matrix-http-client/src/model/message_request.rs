use crate::constants::MessageType;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MessageRequest {
    #[serde(rename = "msgtype")]
    pub msg_type: MessageType,
    pub body: String,
}
