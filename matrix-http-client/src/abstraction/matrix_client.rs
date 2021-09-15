use std::pin::Pin;
use std::future::Future;
use crate::model::{FlowCollection, LoginRequest, LoginResponse, MessageRequest, EventResponse};
use crate::MatrixClientError;
use urlencoding::Encoded;

pub trait TMatrixClient {
    fn get_login<'req>(&'req self) -> Pin<Box<dyn Future<Output=Result<FlowCollection,MatrixClientError>> + 'req>>;

    fn post_login<'req>(&'req self, req: &'req LoginRequest) -> Pin<Box<dyn Future<Output=Result<LoginResponse, MatrixClientError>> + 'req>>;

    fn post_message<'req>(
        &'req self,
        msg: &'req MessageRequest,
        room_id: Encoded<&'req str>,
        access_token: &'req str,
    ) -> Pin<Box<dyn Future<Output=Result<EventResponse, MatrixClientError>> + 'req>>;
}