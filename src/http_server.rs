//! Http Server module
//!
//! This server spawn thread and handle request using the users' handler. The handler
//! provided by the user needs to take a [Request] as parameters and return a [Response].
//!
//! # Example
//! ```Rust
//!use webserv_rs::http_server::HttpServer;
//!use webserv_rs::content_type::ContentType;
//!use webserv_rs::request::Request;
//!use webserv_rs::response::Response;
//!
//!
//!fn handle_client(request: Request) -> Response {
//!    println!("Request: {}", request);
//!    Response::new(200, "Hello, World".as_bytes().to_vec(), vec![], ContentType::TextHtml)
//!}
//!
//!fn main() -> std::io::Result<()> {
//!    let mut server = HttpServer::new("127.0.0.1", 8080)?;
//!    server.run(handle_client)?;
//!
//!    Ok(())
//!}
use crate::request::Request;
use crate::response::Response;
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

    pub fn run(&mut self, handle_client: fn(Request) -> Response) -> std::io::Result<()> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || match stream.peer_addr() {
                        Ok(peer) => Worker::new(stream, peer.to_string()).run(handle_client),
                        Err(e) => eprintln!("Error while creating worker: {e}"),
                    });
                }
                Err(e) => return Err(e),
            };
        }
        Ok(())
    }
}
