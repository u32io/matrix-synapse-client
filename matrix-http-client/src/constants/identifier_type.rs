use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use crate::model::ErrorKind;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum IdentifierType
{
    #[serde(rename = "m.id.user")]
    User,
    #[serde(rename = "m.id.thirdparty")]
    ThirdParty,
    #[serde(rename = "m.id.phone")]
    Phone,
}

impl IdentifierType
{
    const USER: &'static str = "m.id.user";
    const THIRD_PARTY: &'static str = "m.id.thirdparty";
    const PHONE: &'static str = "m.id.phone";

    pub fn as_str(&self) -> &str {
        match self
        {
            IdentifierType::User => Self::USER,
            IdentifierType::ThirdParty => Self::THIRD_PARTY,
            IdentifierType::Phone => Self::PHONE
        }
    }
}

impl TryFrom<&str> for IdentifierType
{
    type Error = ErrorKind;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value
        {
            Self::USER => Ok(Self::User),
            Self::THIRD_PARTY => Ok(Self::ThirdParty),
            Self::PHONE => Ok(Self::Phone),
            _ => Err(ErrorKind::InvalidScheme),
        }
    }
}