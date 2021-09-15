pub use error_code::ErrorCode;
// Models
pub use error_response::ErrorResponse;
pub use event_response::EventResponse;
pub use flow::Flow;
pub use flow::FlowCollection;
pub use login_request::LoginIdentifier;
pub use login_request::LoginRequest;
pub use login_response::LoginResponse;
pub use message_request::MessageRequest;

mod error_code;
mod error_response;
mod event_response;
mod flow;
mod login_request;
mod login_response;
mod message_request;

pub enum ErrorKind {
    InvalidScheme,
    InvalidAuthenticationType,
}
