use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RoomEventType {
    #[serde(rename = "m.room.message")]
    Message,
    #[serde(rename = "m.room.canonical_alias")]
    CanonicalAlias,
    #[serde(rename = "m.room.create")]
    Create,
    #[serde(rename = "m.room.join_rules")]
    JoinRules,
    #[serde(rename = "m.room.member")]
    Member,
    #[serde(rename = "m.room.name")]
    Name,
    #[serde(rename = "m.room.power_levels")]
    PowerLevels,
    #[serde(rename = "m.room.redaction")]
    Redaction,
    #[serde(rename = "m.room.aliases")]
    Aliases,
    #[serde(other)]
    Other,
}

impl RoomEventType {
    /* TODO:
    #[serde(rename = "m.room.canonical_alias")]
    #[serde(rename = "m.room.create")]
    #[serde(rename = "m.room.join_rules")]
    #[serde(rename = "m.room.member")]
    #[serde(rename = "m.room.name")]
    #[serde(rename = "m.room.power_levels")]
    #[serde(rename = "m.room.redaction")]
    #[serde(rename = "m.room.aliases")]
    */
    const MESSAGE: &'static str = "m.room.message";

    pub fn as_str(&self) -> &str {
        match self {
            Self::Message => Self::MESSAGE,
            _ => "",
        }
    }
}
