use serde::{Deserialize};

#[derive(Deserialize)]
pub struct EventResponse
{
    pub event_id: String,
}