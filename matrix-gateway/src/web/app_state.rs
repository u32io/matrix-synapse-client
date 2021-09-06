use actix_web::client::Client;

pub struct AppState
{
    pub http_client: Client
}