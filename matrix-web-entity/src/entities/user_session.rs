use uuid::Uuid;

pub struct UserSession {
    user_id: Uuid,
    access_token: String,
}