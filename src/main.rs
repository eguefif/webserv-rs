use webserv_rs::content_type::ContentType;
use webserv_rs::http_server::HttpServer;
use webserv_rs::request::Request;
use webserv_rs::response::Response;

const BASE_PATH: &str = "./html/dist/";

fn get_index() -> (Vec<u8>, ContentType) {
    let index = std::fs::read_to_string(format!("{}/index.html", BASE_PATH)).unwrap();
    (index.as_bytes().to_vec(), ContentType::TextHtml)
}

fn get_bundle() -> (Vec<u8>, ContentType) {
    let index = std::fs::read_to_string(format!("{}/bundle.js", BASE_PATH)).unwrap();
    (index.as_bytes().to_vec(), ContentType::JS)
}

fn get_asset(uri: &str) -> Option<(Vec<u8>, ContentType)> {
    if uri.contains("css") {
        let css = std::fs::read_to_string(format!("{}/{}", BASE_PATH, uri)).unwrap();
        Some((css.as_bytes().to_vec(), ContentType::CSS))
    } else if uri.contains("favicon") {
        if let Ok(favicon) = std::fs::read(format!("{}/{}", BASE_PATH, uri)) {
            Some((favicon, ContentType::Icon))
        } else {
            None
        }
    } else if uri.contains("images") {
        if let Some(ext) = get_image_extension(uri) {
            if let Ok(image) = std::fs::read(format!("{}/{}", BASE_PATH, uri)) {
                return Some((image, ContentType::Image(ext.to_string())));
            }
        }
        None
    } else {
        None
    }
}

fn get_image_extension(uri: &str) -> Option<&str> {
    if let Some((_, extension)) = uri.rsplit_once(uri) {
        return Some(extension);
    }
    None
}

fn handle_response(request: Request) -> Response {
    println!("uri: {}", request.uri);
    let retval = match request.uri.as_str() {
        "/" => Some(get_index()),
        "/bundle.js" => Some(get_bundle()),
        _ => get_asset(request.uri.as_str()),
    };
    if let Some((body, content_type)) = retval {
        Response::new(200, body.to_vec(), vec![], content_type)
    } else {
        Response::new(400, vec![], vec![], ContentType::TextHtml)
    }
}

fn main() -> std::io::Result<()> {
    let mut server = HttpServer::new("127.0.0.1", 8080)?;
    server.run(handle_response)?;
    Ok(())
}
