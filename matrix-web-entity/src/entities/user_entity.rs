use uuid::Uuid;
use std::rc::Rc;
use super::BotEntity;

/// `UserEntity` represents the human user of this web service, who will be managing various bots.
pub struct UserEntity {
    pub user_id: Uuid,
    pub secret: String,
    pub bots: Vec<Rc<BotEntity>>
}