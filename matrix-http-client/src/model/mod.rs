pub use error_code::ErrorCode as ErrorCode;
// Models
pub use error_response::ErrorResponse as ErrorResponse;
pub use event_response::EventResponse as EventResponse;
pub use flow::Flow as Flow;
pub use flow::FlowCollection as FlowCollection;
pub use login_request::LoginIdentifier as LoginIdentifier;
pub use login_request::LoginRequest as LoginRequest;
pub use login_response::LoginResponse as LoginResponse;
pub use message_request::MessageRequest as MessageRequest;

mod flow;
mod login_response;
mod login_request;
mod error_response;
mod error_code;
mod event_response;
mod message_request;

pub enum ErrorKind
{
    InvalidScheme,
    InvalidAuthenticationType,
}