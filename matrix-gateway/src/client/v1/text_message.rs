use serde::Serialize;

#[derive(Serialize)]
pub struct TextMessage
{
    pub content: TextMessageContent,
    pub event_id: String, // TODO: make this into its own type
    pub origin_server_ts: String, // TODO: find out what this is
    pub room_id: String, // TODO: make a RoomId type
    pub sender: String, // TODO: make a Sender type
    #[serde(rename = "type")]
    pub message_type: String, // TODO: make a custom type (const)
    pub unsigned: Unsigned,
}

#[derive(Serialize)]
pub struct TextMessageContent
{
    pub body: String,
    pub format: String,
    pub formatted_body: String,
    pub msgtype: String,
}

#[derive(Serialize)]
pub struct Unsigned
{
    // TODO: find out what this is
    pub age: u32
}