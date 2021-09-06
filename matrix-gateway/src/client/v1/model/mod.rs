mod flow;
mod identifier_type;
mod authentication_type;

use authentication_type::AuthenticationType as AuthenticationType;
use identifier_type::IdentifierType as IdentifierType;
use flow::Flow as Flow;

pub enum ErrorKind
{
    InvalidScheme,
    InvalidAuthenticationType,
}