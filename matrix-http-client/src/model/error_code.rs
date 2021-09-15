use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ErrorCode {
    #[serde(rename = "M_FORBIDDEN")]
    Forbidden,
    #[serde(rename = "M_UNKNOWN_TOKEN")]
    UnknownToken,
    #[serde(rename = "M_MISSING_TOKEN")]
    MissingToken,
    #[serde(rename = "M_BAD_JSON")]
    BadJson,
    #[serde(rename = "M_NOT_JSON")]
    NotJson,
    #[serde(rename = "M_NOT_FOUND")]
    NotFound,
    #[serde(rename = "M_LIMIT_EXCEEDED")]
    LimitExceeded,
    #[serde(rename = "M_UNKNOWN")]
    Unknown,
    #[serde(rename = "M_UNRECOGNIZED")]
    Unrecognized,
    #[serde(rename = "M_UNAUTHORIZED")]
    Unauthorized,
    #[serde(rename = "M_USER_DEACTIVATED")]
    UserDeactivated,
    #[serde(rename = "M_USER_IN_USE")]
    UserInUse,
    #[serde(rename = "M_INVALID_USERNAME")]
    InvalidUsername,
    #[serde(rename = "M_ROOM_IN_USE")]
    RoomInUse,
    #[serde(rename = "M_INVALID_ROOM_STATE")]
    InvalidRoomState,
    #[serde(rename = "M_THREEPID_IN_USE")]
    ThreePIDInUse,
    #[serde(rename = "M_THREEPID_NOT_FOUND")]
    ThreePIDNotFound,
    #[serde(rename = "M_THREEPID_AUTH_FAILED")]
    ThreePIDAuthFailed,
    #[serde(rename = "M_THREEPID_DENIED")]
    ThreePIDDenied,
    #[serde(rename = "M_SERVER_NOT_TRUSTED")]
    ServerNotTrusted,
    #[serde(rename = "M_UNSUPPORTED_ROOM_VERSION")]
    UnsupportedRoomVersion,
    #[serde(rename = "M_INCOMPATIBLE_ROOM_VERSION")]
    IncompatibleRoomVersion,
    #[serde(rename = "M_BAD_STATE")]
    BadState,
    #[serde(rename = "M_GUEST_ACCESS_FORBIDDEN")]
    GuestAccessForbidden,
    #[serde(rename = "M_CAPTCHA_NEEDED")]
    CaptchaNeeded,
    #[serde(rename = "M_CAPTCHA_INVALID")]
    CaptchaInvalid,
    #[serde(rename = "M_MISSING_PARAM")]
    MissingParam,
    #[serde(rename = "M_INVALID_PARAM")]
    InvalidParam,
    #[serde(rename = "M_TOO_LARGE")]
    TooLarge,
    #[serde(rename = "M_EXCLUSIVE")]
    Exclusive,
    #[serde(rename = "M_RESOURCE_LIMIT_EXCEEDED")]
    ResourceLimitExceeded,
    #[serde(rename = "M_CANNOT_LEAVE_SERVER_NOTICE_ROOM")]
    CannotLeaveServiceNoticeRoom,
}

impl ToString for ErrorCode {
    fn to_string(&self) -> String {
        // TODO: this doesn't need serde. Instead called self.as_str().to_string()
        serde_json::to_string(self).unwrap()
    }
}

impl ErrorCode {
    pub fn as_str(&self) -> &str {
        // TODO: this can be made more efficient with some constants & a match block
        todo!()
    }
}
