use serde::{Serialize};

#[derive(Serialize)]
pub enum MessageType
{
    #[serde(rename = "m.text")]
    Text,
    #[serde(rename = "m.emote")]
    Emote,
    #[serde(rename = "m.notice")]
    Notice,
    #[serde(rename = "m.image")]
    Image,
    #[serde(rename = "m.file")]
    File,
    #[serde(rename = "m.audio")]
    Audio,
    #[serde(rename = "m.location")]
    Location,
    #[serde(rename = "m.video")]
    Video,
}