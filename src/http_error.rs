use std::fmt;

#[derive(Debug)]
pub enum HttpError {
    Error400,
}

impl std::error::Error for HttpError {}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error 400: Bad Request")
    }
}
