use super::ErrorCode;
use serde::Deserialize;

/// Any errors which occur at the Matrix API level MUST return a "standard error response".
#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    #[serde(rename = "errcode")]
    pub code: ErrorCode,
    #[serde(rename = "error")]
    pub message: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn error_message_deserializes_from_json() -> () {
        let json = "{
            \"errcode\": \"M_FORBIDDEN\",
            \"error\": \"Forbidden access, e.g. joining a room without permission, failed login\"
        }";

        let err = serde_json::from_str(json);

        assert!(err.is_ok());
        let err: ErrorResponse = err.unwrap();
        assert_eq!(ErrorCode::Forbidden, err.code);
    }
}
