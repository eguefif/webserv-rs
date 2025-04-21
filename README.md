# Webserv-rs

This is a learning project. Here are the learnings:
* Rust threads and the TCP standard library.
* HTTP protocol [RFC 9112](https://www.rfc-editor.org/rfc/rfc9112.html)
* Websocket protocol [RFC 6455](https://datatracker.ietf.org/doc/html/rfc6455)

It provides a user with an HTTP server library that can handle basic headers parsing and chunked packets.

## Getting Started

### Dependencies

* You need to install [Rust](https://www.rust-lang.org/tools/install)

### Installing

You need to add the library to your dependencies.
```toml
[dependencies]
webserv-rs = { git = "https://github.com/eguefif/webserv-rs.git" }
```

### Example
Here is a Hello World server.

```rust
use webserv_rs::http_server::HttpServer;
use webserv_rs::content_type::ContentType;
use webserv_rs::request::Request;
use webserv_rs::response::Response;


fn handle_client(request: Request) -> Response {
    println!("Request: {}", request);
    Response::new(200, "Hello, World".as_bytes().to_vec(), vec![], ContentType::TextHtml)
}

fn main() -> std::io::Result<()> {
    let mut server = HttpServer::new("127.0.0.1", 8080)?;
    server.run(handle_client)?;

    Ok(())
}

```

## Authors

Emmanuel Guefif

This project is licensed under the [MIT](./LICENCE) License - see the LICENSE.md file for details
