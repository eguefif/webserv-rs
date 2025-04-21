//! HTTP library
//!
//! Provides an HTTP_Server with Request and Response struct.
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
pub mod chunk_handler;
pub mod content_type;
pub mod encoding;
pub mod http_error;
pub mod http_server;
pub mod mock;
pub mod request;
pub mod response;
pub mod worker;
