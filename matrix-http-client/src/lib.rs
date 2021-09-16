mod api_uri_builder;
mod client_config;
mod matrix_client;
mod text_message;

pub mod abstraction;
pub mod model;
pub mod error;
pub mod constants;

pub use abstraction::TMatrixClient;
pub use api_uri_builder::ApiUriBuilder;
pub use client_config::ClientConfig;
pub use matrix_client::MatrixClient;
pub use error::MatrixClientError;