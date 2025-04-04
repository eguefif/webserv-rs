use crate::request::Request;
use std::io::Read;
use std::net::TcpStream;

pub struct Worker {
    socket: TcpStream,
}

impl Worker {
    pub fn new(socket: TcpStream) -> Self {
        Self { socket }
    }

    pub fn run(&mut self, handle_client: fn(Request)) {
        loop {
            if let Ok(request) = self.next() {
                if let Some(request) = request {
                    handle_client(request);
                }
            }
        }
    }

    // TODO: handle remaining if there are
    fn next(&mut self) -> std::io::Result<Option<Request>> {
        let mut buffer = String::new();
        loop {
            let mut tmp = [0u8; 10000];
            let n = self.socket.read(&mut tmp)?;
            if n == 0 {
                return Ok(None);
            }
            buffer.push_str(&String::from_utf8_lossy(&tmp[..n]));
            if is_header_end(&buffer) {
                break;
            }
        }
        Ok(Some(Request::new(&buffer)))
    }
}

fn is_header_end(buffer: &str) -> bool {
    buffer.contains("\r\n\r\n")
}
