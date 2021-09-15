use actix_web::http::{StatusCode};
use crate::model::ErrorResponse;
use std::fmt::{Display, Formatter};
use std::error::Error;

#[derive(Debug)]
pub struct HttpResponseError {
    pub(crate) status: StatusCode,
    pub(crate) body: ErrorResponse,
}

impl HttpResponseError {
    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn body(&self) -> &ErrorResponse {
        &self.body
    }
}

impl Display for HttpResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "status: {} body: {:?}", self.status, self.body)
    }
}

impl Error for HttpResponseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}