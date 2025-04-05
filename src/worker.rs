use crate::request::Request;
use crate::response::Response;
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
            if let Ok(request) = self.get_request() {
                if let Some(request) = request {
                    let response = handle_client(request);
                    self.socket.write_all(&response.as_bytes()).unwrap();
                }
            }
        }
    }

    fn get_request(&mut self) -> std::io::Result<Option<Request>> {
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

    fn read_body(&mut self, buffer: &str, request: &Request) -> std::io::Result<Vec<u8>> {
        if let Some(body_length) = request.get_content_length() {
            let mut body = Vec::with_capacity(body_length);
            body.extend_from_slice(buffer.as_bytes());
            self.socket.read_exact(&mut body)?;
            return Ok(body);
        }
        Ok(vec![0u8; 0])
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
