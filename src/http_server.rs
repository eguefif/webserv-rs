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
                Ok(stream) => thread::spawn(move || match Worker::new(stream) {
                    Ok(mut worker) => worker.run(handle_client),
                    Err(e) => eprintln!("Error while creating worker: {e}"),
                }),
                Err(e) => return Err(e),
            };
        }
        Ok(())
    }
}
