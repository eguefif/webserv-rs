use crate::encoding::{uncompress, Encoding};
use crate::http_error::{handle_error, HttpError};
use crate::request::Request;
use crate::response::Response;
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct Worker {
    socket: TcpStream,
}

impl Worker {
    pub fn new(socket: TcpStream) -> Self {
        Self { socket }
    }

    pub fn run(&mut self, handle_client: fn(Request) -> Response) {
        loop {
            let response = match self.get_request() {
                Ok(request) => {
                    if let Some(request) = request {
                        handle_client(request)
                    } else {
                        break;
                    }
                }
                Err(error) => handle_error(error),
            };
            self.socket.write_all(&response.as_bytes()).unwrap();
        }
    }

    fn get_request(&mut self) -> Result<Option<Request>, Box<dyn Error>> {
        let mut buffer = String::new();
        let mut request;
        loop {
            let mut tmp = [0u8; 1024];
            let n = self.socket.read(&mut tmp)?;
            if n == 0 {
                return Ok(None);
            }
            buffer.push_str(&String::from_utf8_lossy(&tmp[..n]));
            if is_header_finished(&buffer) {
                request = Request::new(&buffer);
                if request.is_body() {
                    let buffer = prepare_buffer_for_body(buffer);
                    request.body = self.read_body(&buffer, &request)?;
                }
                break;
            }
        }
        Ok(Some(request))
    }

    fn read_body(&mut self, buffer: &str, request: &Request) -> Result<Vec<u8>, Box<dyn Error>> {
        if let Some(encoding) = request.get_value("Transfer-Encoding") {
            self.handle_encoded_body(buffer, request, encoding)
        } else if let Some(body_length) = request.get_content_length() {
            self.handle_content_length_body(buffer, body_length)
        } else {
            Err(Box::new(HttpError::Error400))
        }
    }

    fn handle_encoded_body(
        &mut self,
        buffer: &str,
        request: &Request,
        encoding_field: &str,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        // NOTE:: How to proceed
        // First part will contain the encoding algorithm
        // If chunked here, we get by chunk end decode.
        // If no chunked we get everything and decode
        if let Some(encoding) = get_encoding(encoding_field) {
            if request.is_chunked() {
                let retval = vec![0u8; 0];
                return Ok(retval);
            } else if let Some(length) = request.get_content_length() {
                let body = self.handle_content_length_body(buffer, length)?;
                let retval = uncompress(&body, encoding);
                return Ok(retval);
            }
        }
        Err(Box::new(HttpError::Error400))
    }

    fn handle_content_length_body(
        &mut self,
        buffer: &str,
        body_length: usize,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let remaining_size = body_length - buffer.len();
        let mut remaining_buffer = Vec::with_capacity(remaining_size);
        self.socket.read_exact(&mut remaining_buffer)?;

        let mut body = Vec::with_capacity(body_length);
        body.extend_from_slice(buffer.as_bytes());
        body.extend_from_slice(&remaining_buffer);

        return Ok(body);
    }
}

fn is_header_finished(buffer: &str) -> bool {
    buffer.contains("\r\n\r\n")
}

fn prepare_buffer_for_body(buffer: String) -> String {
    let mut splits = buffer.split("\r\n\r\n");
    splits.next().unwrap();
    if let Some(remaining) = splits.next() {
        remaining.to_string()
    } else {
        String::new()
    }
}

fn get_encoding(encoding: &str) -> Option<Encoding> {
    match encoding.split(" ").next().unwrap().trim() {
        "gzip" => Some(Encoding::Gzip),
        "deflate" => Some(Encoding::Deflate),
        _ => None,
    }
}
