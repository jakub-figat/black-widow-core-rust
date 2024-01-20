use std::error::Error;

#[derive(Debug)]
pub(crate) struct ValidationError(pub String);


impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Box<dyn Error>> for ValidationError {
    fn from(error: Box<dyn Error>) -> ValidationError {
        ValidationError(error.to_string())
    }
}

impl Error for ValidationError {}