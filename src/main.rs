use webserv_rs::http_server::HttpServer;
use webserv_rs::request::Request;

fn handle_response(request: Request) {
    println!("Request:\n {}", request);
}

fn main() -> std::io::Result<()> {
    let mut server = HttpServer::new("127.0.0.1", 8080)?;
    server.run(handle_response)?;
    Ok(())
}
