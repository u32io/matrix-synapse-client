mod flow;
mod identifier_type;
mod authentication_type;
mod login_response;
mod login_request;
mod room_event_type;

pub use authentication_type::AuthenticationType as AuthenticationType;
pub use identifier_type::IdentifierType as IdentifierType;
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