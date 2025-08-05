use std::{
    error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub struct Error {
    description: &'static str,
    code: i32,
}

impl Error {
    pub fn new(description: &'static str, code: i32) -> Error {
        Error { description, code }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}: code {}", self.description, self.code)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.description
    }
}
