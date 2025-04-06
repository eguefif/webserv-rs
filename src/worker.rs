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
    leftover: Vec<u8>,
    peer: String,
}

impl Worker {
    pub fn new(socket: TcpStream) -> Result<Self, Box<dyn Error>> {
        let peer = socket.peer_addr()?.to_string();
        Ok(Self {
            socket,
            leftover: vec![0u8; 0],
            peer,
        })
    }

    pub fn run(&mut self, handle_client: fn(Request) -> Response) {
        println!("New connection end with : {}", self.peer);
        loop {
            let response = match self.get_request() {
                Ok(request) => {
                    if let Some(request) = request {
                        handle_client(request)
                    } else {
                        println!("Connection end with : {}", self.peer);
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
        if self.leftover.len() > 0 {
            buffer.extend_from_slice(&self.leftover);
        }
        let request = loop {
            let mut tmp = [0u8; 1024];
            let n = self.socket.read(&mut tmp)?;
            if n == 0 {
                return Ok(None);
            }
            buffer.extend_from_slice(&tmp[..n]);
            if let Some(index) = get_double_crcn_index(&buffer) {
                let mut request = Request::new(&String::from_utf8_lossy(&buffer[..index]));
                if request.is_body() {
                    let buffer = &buffer[index + 4..];
                    request.body = self.read_body(&buffer, &request)?;
                }
                break request;
            }
        };
        Ok(Some(request))
    }

    fn read_body(&mut self, buffer: &[u8], request: &Request) -> Result<Vec<u8>, Box<dyn Error>> {
        let body = self.get_body(buffer, request)?;
        self.uncompress(&body, &request)
    }

    fn get_body(&mut self, buffer: &[u8], request: &Request) -> Result<Vec<u8>, Box<dyn Error>> {
        if let Some(encoding) = request.get_value("Transfer-Encoding") {
            self.handle_chunked_body(buffer, encoding)
        } else if let Some(body_length) = request.get_content_length() {
            self.handle_content_length_body(buffer, body_length)
        } else {
            Err(Box::new(HttpError::Error400))
        }
    }

    fn uncompress(&mut self, body: &[u8], request: &Request) -> Result<Vec<u8>, Box<dyn Error>> {
        if let Some(encoding) = request.get_value("Content-Encoding") {
            if let Some(encoding) = get_encoding(encoding) {
                uncompress(&body, encoding)
            } else {
                return Err(Box::new(HttpError::Error415));
            }
        } else {
            Ok(body.to_vec())
        }
    }

    fn handle_chunked_body(
        &mut self,
        leftover: &[u8],
        encoding_field: &str,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        if encoding_field.to_lowercase().contains("chunked") {
            let mut chunk_handler = ChunkHandler::new(leftover);
            if !chunk_handler.is_body_ready() {
                loop {
                    let mut tmp = [0u8; 1024];
                    let n = self.socket.read(&mut tmp)?;
                    chunk_handler.parse_chunks(&tmp[..n]);
                    if chunk_handler.is_body_ready() {
                        break;
                    }
                }
            }
            return Ok(chunk_handler.body);
        }
        Err(Box::new(HttpError::Error400))
    }

    fn handle_content_length_body(
        &mut self,
        buffer: &[u8],
        body_length: usize,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        if body_length == buffer.len() {
            Ok(buffer.to_vec())
        } else {
            let remaining_buffer = self.read_remaining(buffer, body_length)?;
            self.assemble_buffer_with_remaining(buffer, &remaining_buffer, body_length)
        }
    }

    fn read_remaining(
        &mut self,
        buffer: &[u8],
        body_length: usize,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let remaining_size = body_length - buffer.len();
        if remaining_size == 0 {
            return Ok(buffer.to_vec());
        }
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

fn get_encoding(encoding: &str) -> Option<Encoding> {
    match encoding.split(" ").next().unwrap().trim() {
        "gzip" => Some(Encoding::Gzip),
        "deflate" => Some(Encoding::Deflate),
        _ => None,
    }
}
