use crate::model::BasicMessage;
use std::rc::Rc;
use matrix_http_client::{MatrixClient, MatrixClientError};

pub struct MessageService
{
    matrix_client: Rc<MatrixClient>,
}

impl MessageService
{
    pub fn new(matrix_client: Rc<MatrixClient>) -> Self
    {
        Self {
            matrix_client
        }
    }

    pub fn send(&self, msg: BasicMessage) -> Result<(),MatrixClientError>
    {
        todo!()
    }
}