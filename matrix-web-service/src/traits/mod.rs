use std::future::Future;

trait TMessageService {
    fn send_message() -> dyn Future;
}