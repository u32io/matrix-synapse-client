use super::AuthenticationType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Flow
{
    #[serde(rename = "type")]
    pub authentication_type: AuthenticationType,
}

#[cfg(test)]
mod test
{
    use super::Flow;
    use super::AuthenticationType;

    #[test]
    fn flow_deserializes_from_json()
    {
        let json = "
        {
            \"type\": \"m.login.password\"
        }";

        let flow = serde_json::from_str(&json);
        assert!(flow.is_ok());

        let flow: Flow = flow.unwrap();
        assert_eq!(AuthenticationType::Password, flow.authentication_type);
    }
}