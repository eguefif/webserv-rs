use crate::content_type::ContentType;
use crate::response::Response;
use std::fmt;
use std::io::Read;

trait ErrorResponse {
    fn response_from_error(&self) -> Response;
}

#[derive(Debug)]
pub enum HttpError {
    Error400,
    Error404,
    Error415,
}

impl std::error::Error for HttpError {}

impl ErrorResponse for Box<HttpError> {
    fn response_from_error(&self) -> Response {
        match **self {
            HttpError::Error400 => {
                let mut file = std::fs::File::open("./html/400.html").unwrap();
                let mut body = Vec::with_capacity(400);
                let _ = file.read_to_end(&mut body);
                Response::new(400, body, vec![], ContentType::TextHtml)
            }
            HttpError::Error404 => {
                let mut file = std::fs::File::open("./html/404.html").unwrap();
                let mut body = Vec::with_capacity(400);
                let _ = file.read_to_end(&mut body);
                Response::new(404, body, vec![], ContentType::TextHtml)
            }
            HttpError::Error415 => {
                let mut file = std::fs::File::open("./html/415.html").unwrap();
                let mut body = Vec::with_capacity(400);
                let _ = file.read_to_end(&mut body);
                Response::new(404, body, vec![], ContentType::TextHtml)
            }
        }
    }
}

impl ErrorResponse for Box<dyn std::error::Error> {
    fn response_from_error(&self) -> Response {
        let mut file = std::fs::File::open("./html/500.html").unwrap();
        let mut body = Vec::with_capacity(400);
        let _ = file.read_to_end(&mut body);
        Response::new(500, body, vec![], ContentType::TextHtml)
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error 400: Bad Request")
    }
}

pub fn handle_error(error: Box<dyn std::error::Error>) -> Response {
    eprintln!("Error: {error}");
    error.response_from_error()
}
