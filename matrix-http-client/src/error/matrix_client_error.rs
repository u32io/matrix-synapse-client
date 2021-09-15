use std::error::Error;
use actix_web::client::{SendRequestError, PayloadError};
use std::fmt::{Display, Formatter};
use crate::error::HttpResponseError;
use crate::abstraction::GetError;

#[derive(Debug)]
pub enum MatrixClientError {
    SendRequestError(SendRequestError),
    PayloadErr(PayloadError),
    JsonDeserializationError(serde_json::Error),
    HttpResponseError(HttpResponseError),
    Unknown,
}

impl Display for MatrixClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MatrixClientError")
    }
}

impl Error for MatrixClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MatrixClientError::SendRequestError(e) => Some(e),
            MatrixClientError::PayloadErr(e) => Some(e),
            MatrixClientError::JsonDeserializationError(e) => Some(e),
            MatrixClientError::HttpResponseError(e) => Some(e),
            _ => None,
        }
    }
}

impl GetError<HttpResponseError> for MatrixClientError {
    fn get_error(&self) -> Option<&HttpResponseError> {
        match self {
            MatrixClientError::HttpResponseError(e) => Some(&e),
            _ => None,
        }
    }
}