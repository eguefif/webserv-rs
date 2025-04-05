#[allow(dead_code)]
pub struct Response {
    version: String,
    status: u32,
    reason: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl Response {
    pub fn new(status: u32, body: Vec<u8>, headers: Vec<(String, String)>) -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status,
            reason: reason_phrase(status),
            body,
            headers,
        }
    }
}

fn reason_phrase(status: u32) -> String {
    match status {
        // 1xx Informational
        100 => "Continue".to_string(),
        101 => "Switching Protocols".to_string(),

        // 2xx Success
        200 => "OK".to_string(),
        201 => "Created".to_string(),
        202 => "Accepted".to_string(),
        203 => "Non-Authoritative Information".to_string(),
        204 => "No Content".to_string(),
        205 => "Reset Content".to_string(),
        206 => "Partial Content".to_string(),

        // 3xx Redirection
        300 => "Multiple Choices".to_string(),
        301 => "Moved Permanently".to_string(),
        302 => "Found".to_string(),
        303 => "See Other".to_string(),
        304 => "Not Modified".to_string(),
        305 => "Use Proxy".to_string(),
        306 => "(Unused)".to_string(),
        307 => "Temporary Redirect".to_string(),
        308 => "Permanent Redirect".to_string(),

        // 4xx Client Error
        400 => "Bad Request".to_string(),
        401 => "Unauthorized".to_string(),
        402 => "Payment Required".to_string(),
        403 => "Forbidden".to_string(),
        404 => "Not Found".to_string(),
        405 => "Method Not Allowed".to_string(),
        406 => "Not Acceptable".to_string(),
        407 => "Proxy Authentication Required".to_string(),
        408 => "Request Timeout".to_string(),
        409 => "Conflict".to_string(),
        410 => "Gone".to_string(),
        411 => "Length Required".to_string(),
        412 => "Precondition Failed".to_string(),
        413 => "Content Too Large".to_string(),
        414 => "URI Too Long".to_string(),
        415 => "Unsupported Media Type".to_string(),
        416 => "Range Not Satisfiable".to_string(),
        417 => "Expectation Failed".to_string(),
        418 => "I'm a tea pot".to_string(),
        421 => "Misdirected Request".to_string(),
        422 => "Unprocessable Content".to_string(),
        426 => "Upgrade Required".to_string(),

        // 5xx Server Error
        500 => "Internal Server Error".to_string(),
        501 => "Not Implemented".to_string(),
        502 => "Bad Gateway".to_string(),
        503 => "Service Unavailable".to_string(),
        504 => "Gateway Timeout".to_string(),
        505 => "HTTP Version Not Supported".to_string(),

        // Default
        _ => "Unknown Status Code".to_string(),
    }
}
