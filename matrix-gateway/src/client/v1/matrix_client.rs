use crate::web::AppState;
use actix_web::client::Client;
use actix_web::http::Uri;

struct MatrixClient
{
    base_uri: Uri,
    http_client: Client,
}

impl From<AppState> for MatrixClient
{
    fn from(app_state: AppState) -> Self {
        MatrixClient {
            base_uri: Uri::from_static("https://matrix.allyourcoinsarebelongtous.xyz"),
            http_client: app_state.http_client
        }
    }
}

impl MatrixClient
{
    pub async fn send_message(&self) -> ()
    {
        self.http_client.put()
    }
}