use crate::chunk_handler::ChunkHandler;
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
        let mut buffer = vec![0u8; 1024];
        let mut request;
        loop {
            let mut tmp = [0u8; 1024];
            let n = self.socket.read(&mut tmp)?;
            if n == 0 {
                return Ok(None);
            }
            buffer.extend_from_slice(&tmp[..n]);
            if let Some(index) = get_double_crcn_index(&buffer) {
                request = Request::new(&String::from_utf8_lossy(&buffer[..index]));
                if request.is_body() {
                    println!("size: {}\n{:?}", buffer.len(), buffer);
                    let buffer = prepare_buffer_for_body(buffer);
                    println!("Prepared buffer({}: {:?}", buffer.len(), buffer);
                    request.body = self.read_body(&buffer, &request)?;
                }
                break;
            }
        }
        Ok(Some(request))
    }

    fn read_body(&mut self, buffer: &[u8], request: &Request) -> Result<Vec<u8>, Box<dyn Error>> {
        let body = if let Some(encoding) = request.get_value("Transfer-Encoding") {
            self.handle_encoded_body(buffer, request, encoding)
        } else if let Some(body_length) = request.get_content_length() {
            self.handle_content_length_body(buffer, body_length)
        } else {
            return Err(Box::new(HttpError::Error400));
        }?;
        let body = if let Some(encoding) = request.get_value("Content-Encoding") {
            if let Some(encoding) = get_encoding(encoding) {
                uncompress(&body, encoding)
            } else {
                return Err(Box::new(HttpError::Error415));
            }
        } else {
            body
        };

        Ok(body)
    }

    fn handle_encoded_body(
        &mut self,
        buffer: &[u8],
        request: &Request,
        encoding_field: &str,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        if let Some(encoding) = get_encoding(encoding_field) {
            if request.is_chunked() {
                let mut chunk_handler = ChunkHandler::new();
                let retval = vec![0u8; 0];
                return Ok(retval);
            } else if let Some(length) = request.get_content_length() {
                let body = self.handle_content_length_body(buffer, length)?;
                return Ok(body);
            }
        }
        Err(Box::new(HttpError::Error400))
    }

    fn handle_content_length_body(
        &mut self,
        buffer: &[u8],
        body_length: usize,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let remaining_buffer = self.read_remaining(buffer, body_length)?;
        self.assemble_buffer_with_remaining(buffer, &remaining_buffer, body_length)
    }

    fn read_remaining(
        &mut self,
        buffer: &[u8],
        body_length: usize,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        println!("body: {}, buffer: {}", body_length, buffer.len());
        if body_length == buffer.len() {
            return Ok(buffer.to_vec());
        }
        let remaining_size = body_length - buffer.len();
        let mut remaining_buffer = Vec::with_capacity(remaining_size);
        self.socket.read_exact(&mut remaining_buffer)?;
        Ok(remaining_buffer)
    }

    fn assemble_buffer_with_remaining(
        &self,
        buffer: &[u8],
        remaining_buffer: &[u8],
        body_length: usize,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut body = Vec::with_capacity(body_length);
        body.extend_from_slice(buffer);
        body.extend_from_slice(&remaining_buffer);
        Ok(body)
    }
}

fn is_header_finished(buffer: &[u8]) -> bool {
    if let Some(_) = get_double_crcn_index(buffer) {
        true
    } else {
        false
    }
}

fn get_double_crcn_index(buffer: &[u8]) -> Option<usize> {
    for (i, _) in buffer.iter().enumerate() {
        if i + 4 < buffer.len() {
            if String::from_utf8_lossy(&buffer[i..i + 4]) == "\r\n\r\n" {
                return Some(i);
            }
        } else {
            break;
        }
    }
    None
}

fn prepare_buffer_for_body(buffer: String) -> Vec<u8> {
    let mut splits = buffer.split("\r\n\r\n");
    splits.next().unwrap();
    if let Some(remaining) = splits.next() {
        remaining.as_bytes().to_vec()
    } else {
        Vec::new()
    }
}

fn get_encoding(encoding: &str) -> Option<Encoding> {
    match encoding.split(" ").next().unwrap().trim() {
        "gzip" => Some(Encoding::Gzip),
        "deflate" => Some(Encoding::Deflate),
        _ => None,
    }
}
