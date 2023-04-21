use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DockerAPIError<'a> {
    pub api_type: &'a str,
    pub message: &'a str,
}

impl fmt::Display for DockerAPIError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error for {} endpoint: {}", self.api_type, self.message)
    }
}

impl Error for DockerAPIError<'_> {}
