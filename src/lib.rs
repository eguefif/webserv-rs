use std::net::{TcpListener, TcpStream};

pub struct HttpServer {
    listener: TcpListener,
}

impl HttpServer {
    pub fn new(ip: &str, port: u32) -> std::io::Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(format!("{}:{}", ip, port))?,
        })
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        for stream in self.listener.incoming() {
            handle_client(stream?);
        }
        Ok(())
    }
}

fn handle_client(_stream: TcpStream) {}
