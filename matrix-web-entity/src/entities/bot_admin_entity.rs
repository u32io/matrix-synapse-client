use uuid::Uuid;
use std::rc::Rc;
use super::BotEntity;

/// `BotAdminEntity` represents the human user of this web service, who will be managing various bots.
pub struct BotAdminEntity {
    pub user_id: Uuid,
    pub name: String,
    pub home_sever: String,
    pub bots: Vec<BotEntity>
}