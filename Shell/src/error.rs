use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MyError {
    details: String,
}

impl MyError {
    pub fn new(msg: &str) -> Box<Self> {
        Box::new(MyError {
            details: msg.to_string(),
        })
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}
