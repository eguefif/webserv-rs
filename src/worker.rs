use crate::chunk_handler::ChunkHandler;
use crate::encoding::{uncompress, Encoding};
use crate::http_error::{handle_error, HttpError};
use crate::request::Request;
use crate::response::Response;
use std::error::Error;
use std::io::{Read, Write};

const MAX_HEADER_SIZE: usize = 16_000;
pub const MAX_BODY_SIZE: usize = 1024 * 1024;

pub struct Worker<T: Read + Write> {
    socket: T,
    leftover: Vec<u8>,
    peer: String,
}

impl<T: Read + Write> Worker<T> {
    pub fn new(socket: T, peer: String) -> Self {
        Self {
            socket,
            leftover: vec![0u8; 0],
            peer,
        }
    }

    pub fn run(&mut self, handle_client: fn(Request) -> Response) {
        println!("New connection end with : {}", self.peer);
        loop {
            if let Some(response) = self.get_response(handle_client) {
                if let Err(e) = self.socket.write_all(&response.as_bytes()) {
                    eprintln!("Error while writing in socket({}): {e}", self.peer);
                    break;
                }
                if response.is_error_status() {
                    break;
                }
            } else {
                break;
            }
        }
        println!("Connection end with : {}", self.peer);
    }

    fn get_response(&mut self, handle_client: fn(Request) -> Response) -> Option<Response> {
        match self.get_request() {
            Ok(request) => {
                if let Some(request) = request {
                    return Some(handle_client(request));
                } else {
                    return None;
                }
            }
            Err(error) => return Some(handle_error(error)),
        }
    }

    fn get_request(&mut self) -> Result<Option<Request>, Box<dyn Error>> {
        let mut buffer = vec![0u8; 0];
        if self.leftover.len() > 0 {
            buffer.extend_from_slice(&self.leftover);
        }
        loop {
            let mut tmp = [0u8; 1024];
            let n = self.socket.read(&mut tmp)?;
            if n == 0 {
                return Ok(None);
            }
            buffer.extend_from_slice(&tmp[..n]);
            if let Some(index) = get_double_crcn_index(&buffer) {
                let request = self.process_packet(index, &buffer)?;
                return Ok(Some(request));
            }
            if buffer.len() > MAX_HEADER_SIZE {
                return Err(Box::new(HttpError::Error400));
            }
        }
    }

    fn process_packet(&mut self, index: usize, buffer: &[u8]) -> Result<Request, Box<dyn Error>> {
        let mut request = Request::new(&String::from_utf8_lossy(&buffer[..index]));
        if request.is_body() {
            let buffer = &buffer[index + 4..];
            request.body = self.read_body(&buffer, &request)?;
        } else {
            self.leftover = buffer[index + 4..].to_vec();
        }
        Ok(request)
    }

    fn read_body(&mut self, buffer: &[u8], request: &Request) -> Result<Vec<u8>, Box<dyn Error>> {
        let body = self.get_body(buffer, request)?;
        self.uncompress(&body, &request)
    }

    fn get_body(&mut self, buffer: &[u8], request: &Request) -> Result<Vec<u8>, Box<dyn Error>> {
        if let Some(encoding) = request.get_value("Transfer-Encoding") {
            self.handle_chunked_body(buffer, encoding)
        } else if let Some(body_length) = request.get_content_length() {
            if body_length > MAX_BODY_SIZE {
                return Err(Box::new(HttpError::Error413));
            }
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
        // TODO: this header field (encoding field) can also contain encoding like gzip or deflate
        if encoding_field.to_lowercase().contains("chunked") {
            let mut chunk_handler = ChunkHandler::new(leftover)?;
            if !chunk_handler.is_body_ready() {
                loop {
                    let mut tmp = [0u8; 1024];
                    let n = self.socket.read(&mut tmp)?;
                    chunk_handler.parse_chunks(&tmp[..n])?;
                    if chunk_handler.is_body_ready() {
                        if chunk_handler.leftover.len() > 0 {
                            self.leftover = chunk_handler.leftover;
                        }
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
        if body_length < buffer.len() {
            self.leftover = buffer[body_length..].to_vec();
            Ok(buffer[..body_length].to_vec())
        } else if body_length == buffer.len() {
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
        let mut remaining_buffer = vec![0u8; remaining_size];
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

#[cfg(test)]
mod test {
    use crate::content_type::ContentType;
    use crate::mock::{TcpStreamMock, CHUNKED, EXPECTED, REGULAR_PACKET};

    use super::*;

    fn get_worker(data: &[&[u8]]) -> Worker<TcpStreamMock> {
        let socket = TcpStreamMock::new(data);
        Worker {
            peer: "127.0.0.1:8080".to_string(),
            socket,
            leftover: vec![0u8; 0],
        }
    }

    fn handle_client_mock(request: Request) -> Response {
        Response::new(200, request.as_bytes(), vec![], ContentType::TextHtml)
    }

    #[test]
    fn it_should_parse_request_body() {
        let mut worker = get_worker(REGULAR_PACKET);
        worker.run(handle_client_mock);

        let request = String::from_utf8_lossy(&worker.socket.receive);
        let mut request_splits = request.split("\r\n\r\n");
        let _response_header = request_splits.next().unwrap();
        let request_header = request_splits.next().unwrap();
        let request_body = request_splits.next().unwrap();

        let mut expected_splits = EXPECTED.split("\r\n\r\n");
        let expected_header = expected_splits.next().unwrap();
        let expected_body = expected_splits.next().unwrap();

        assert_eq!(request_body, expected_body);
        for header in request_header.split("\r\n").into_iter() {
            assert!(expected_header.contains(&header));
        }
    }

    #[test]
    fn it_should_parse_request_chunked_body() {
        let mut worker = get_worker(CHUNKED);
        worker.run(handle_client_mock);

        let request = String::from_utf8_lossy(&worker.socket.receive);
        let mut request_splits = request.split("\r\n\r\n");
        request_splits.next().unwrap();
        request_splits.next().unwrap();
        let request_body = request_splits.next().unwrap();

        assert_eq!(request_body, "HelloWorldfromthesky");
    }
}
