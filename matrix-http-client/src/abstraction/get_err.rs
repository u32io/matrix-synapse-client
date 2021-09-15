use std::error::Error;

pub trait GetError<T: Error> {
    fn get_error(&self) -> Option<&T>;
}