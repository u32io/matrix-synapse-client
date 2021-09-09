mod flow;
mod identifier_type;
mod authentication_type;
mod login_response;
mod login_request;
mod room_event_type;
mod error_response;
mod error_code;
mod event_response;
mod message_request;
mod message_type;

// TODO: move enums to their own directory
// Enums
pub use authentication_type::AuthenticationType as AuthenticationType;
pub use identifier_type::IdentifierType as IdentifierType;
pub use error_code::ErrorCode as ErrorCode;
pub use message_type::MessageType as MessageType;

// Models
pub use error_response::ErrorResponse as ErrorResponse;
pub use event_response::EventResponse as EventResponse;
pub use message_request::MessageRequest as MessageRequest;
pub use flow::Flow as Flow;
pub use flow::FlowCollection as FlowCollection;
pub use login_request::LoginRequest as LoginRequest;
pub use login_request::LoginIdentifier as LoginIdentifier;
pub use login_response::LoginResponse as LoginResponse;
pub use room_event_type::RoomEventType as RoomEventType;

pub enum ErrorKind
{
    InvalidScheme,
    InvalidAuthenticationType,
}