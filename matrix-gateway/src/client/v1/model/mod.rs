mod flow;
mod identifier_type;
mod authentication_type;

use authentication_type::AuthenticationType as AuthenticationType;
use identifier_type::IdentifierType as IdentifierType;
pub use flow::Flow as Flow;
pub use flow::FlowCollection as FlowCollection;

pub enum ErrorKind
{
    InvalidScheme,
    InvalidAuthenticationType,
}