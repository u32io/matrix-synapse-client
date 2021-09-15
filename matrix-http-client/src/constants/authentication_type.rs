use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AuthenticationType
{
    #[serde(rename = "m.login.password")]
    Password,
    #[serde(rename = "m.login.recaptcha")]
    ReCaptcha,
    #[serde(rename = "m.login.oauth2")]
    OAuth2,
    #[serde(rename = "m.login.sso")]
    SSO,
    #[serde(rename = "m.login.email.identity")]
    EmailIdentity,
    #[serde(rename = "m.login.msisdn")]
    MSISDN,
    #[serde(rename = "m.login.token")]
    Token,
    #[serde(rename = "m.login.dummy")]
    Dummy,
    #[serde(other)]
    Other
}