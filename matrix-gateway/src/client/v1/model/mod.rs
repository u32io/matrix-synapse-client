use std::convert::TryFrom;

mod flow;
mod identifier_type;

use identifier_type::IdentifierType as IdentifierType;

pub enum ErrorKind
{
    InvalidScheme,
}