use std::io::Read;
use std::net::TcpStream;

pub struct Worker {
    socket: TcpStream,
}

impl Worker {
    pub fn new(socket: TcpStream) -> Self {
        Self { socket }
    }

    pub fn handle_client(&mut self) {
        loop {
            let mut buffer = [0u8; 10000];
            match self.socket.read(&mut buffer) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }
                    self.handle_read(n, &buffer);
                }
                Err(e) => eprintln!("Error in worker: {e}"),
            }
        }
    }

    fn handle_read(&mut self, n: usize, buffer: &[u8]) {
        println!("{}: {}", n, String::from_utf8_lossy(buffer));
    }
}
