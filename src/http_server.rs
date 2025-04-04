use crate::request::Request;
use crate::worker::Worker;
use std::net::TcpListener;
use std::thread;

pub struct HttpServer {
    listener: TcpListener,
}

impl HttpServer {
    pub fn new(ip: &str, port: u32) -> std::io::Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(format!("{}:{}", ip, port))?,
        })
    }

    pub fn run(&mut self, handle_client: fn(Request)) -> std::io::Result<()> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || Worker::new(stream).run(handle_client));
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}
