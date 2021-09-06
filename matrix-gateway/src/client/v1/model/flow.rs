use super::IdentifierType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Flow
{
    #[serde(rename = "type")]
    pub identifier_type: IdentifierType,
}

#[cfg(test)]
mod test
{
    use super::Flow;
    use super::IdentifierType;

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
        assert_eq!(IdentifierType::User, flow.identifier_type);
    }
}