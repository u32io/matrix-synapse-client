mod matrix_client;
mod text_message;
mod model;
mod client_config;
mod api_uri_builder;

use model::{Flow, FlowCollection, AuthenticationType};

pub use api_uri_builder::ApiUriBuilder as ApiUriBuilder;
pub use client_config::ClientConfig as ClientConfig;
pub use matrix_client::MatrixClient as MatrixClient;

#[cfg(test)]
mod test
{
    use super::*;
    use std::convert::TryFrom;
    use std::path::Path;
    use actix_web::client::Client;

    fn init_matrix_client() -> MatrixClient
    {
        let config = ClientConfig::try_from(Path::new(".client.json")).unwrap();
        let api_uri_builder = ApiUriBuilder::new(config.authority.as_str(), config.client_api.as_str()).unwrap();
        let actix_http_client = Client::default();

        MatrixClient::new(api_uri_builder, actix_http_client)
    }

    #[actix_rt::test]
    async fn matrix_client_returns_flow_on_successful_get_login()
    {
        let matrix = init_matrix_client();

        let mut flow = matrix.get_login().await;

        assert!(flow.len() >= 1);
    }
}