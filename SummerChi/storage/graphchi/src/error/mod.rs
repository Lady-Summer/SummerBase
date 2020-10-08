use std::error::Error;
use std::borrow::Borrow;
use std::io::ErrorKind;

pub struct ShardIndexOutOfBound;

impl Error for ShardIndexOutOfBound {}

pub struct IntervalLoadError {
    err_msg: String,
    err_cause: Option<std::io::Error>
}

impl Error for IntervalLoadError {
    fn description(&self) -> &str {
        self.err_msg.as_str()
    }

    fn cause(&self) -> Option<&std::io::Error> {
        self.err_cause.map(|x| x.borrow())
    }
}

impl IntervalLoadError {
    pub fn new(msg: &String, err: &std::io::Error) -> Self {
        IntervalLoadError {
            err_msg: msg.into_string(),
            err_cause: Some(err).map(|x| x.into())
        }
    }
}

impl Into<std::io::Error> for IntervalLoadError {
    fn into(self) -> std::io::Error {
        let self_ref = self.borrow();
        match self.err_cause {
            Some(cause) => std::io::Error::new(cause.kind(), self_ref),
            None => std::io::Error::new(ErrorKind::NotFound, self_ref)
        }
    }
}
