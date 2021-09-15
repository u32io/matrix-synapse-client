use crate::constants::AuthenticationType;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Flow {
    #[serde(rename = "type")]
    pub authentication_type: AuthenticationType,
}

#[derive(Deserialize)]
pub struct FlowCollection {
    pub flows: Vec<Flow>,
}

#[cfg(test)]
mod test {
    use super::AuthenticationType;
    use super::Flow;

    #[test]
    fn flow_deserializes_from_json() {
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
