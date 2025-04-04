use webserv_rs::http_server::HttpServer;

fn main() -> std::io::Result<()> {
    let mut server = HttpServer::new("127.0.0.1", 8080)?;
    server.run()?;
    Ok(())
}
