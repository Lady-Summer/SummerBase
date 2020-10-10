use std::error::Error;
use std::borrow::Borrow;
use std::io::{ErrorKind};
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Debug)]
pub struct IntervalLoadError {
    err_msg: String,
    err_cause: Option<std::io::Error>
}

impl IntervalLoadError {
    pub fn new(msg: &String, err: std::io::Error) -> Self {
        IntervalLoadError {
            err_msg: msg.clone(),
            err_cause: Some(err)
        }
    }
}

impl Into<std::io::Error> for IntervalLoadError {
    fn into(self) -> std::io::Error {
        match self.err_cause {
            Some(cause) => std::io::Error::new(cause.kind(), cause),
            None => std::io::Error::new(
                ErrorKind::NotFound,
                std::io::Error::new(ErrorKind::Other, "Other Error Kind")
            )
        }
    }
}
