use webserv_rs::content_type::ContentType;
use webserv_rs::http_server::HttpServer;
use webserv_rs::request::Request;
use webserv_rs::response::Response;

fn handle_response(request: Request) -> Response {
    println!("Request:\n {}", request);
    let content = std::fs::read_to_string("./index.html").unwrap();
    Response::new(
        200,
        content.as_bytes().to_vec(),
        vec![],
        ContentType::TextHtml,
    )
}

fn main() -> std::io::Result<()> {
    let mut server = HttpServer::new("127.0.0.1", 8080)?;
    server.run(handle_response)?;
    Ok(())
}
