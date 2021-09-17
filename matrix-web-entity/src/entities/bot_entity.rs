use uuid::Uuid;
/// `BotEntity` is owned by a `UserEntity`, where a user may have many bots.
/// The `BotEntity` is the actual entity that is impersonated when messages are sent to the matrix
/// synapse api.
pub struct BotEntity {
    pub bot_id: Uuid,
    pub name: String,
    pub home_sever: String,
    pub password: String,
}