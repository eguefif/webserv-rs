use crate::content_type::ContentType;
use chrono::prelude::*;

#[allow(dead_code)]
pub struct Response {
    version: String,
    status: u32,
    reason: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl Response {
    pub fn new(
        status: u32,
        body: Vec<u8>,
        headers: Vec<(String, String)>,
        content_type: ContentType,
    ) -> Self {
        let headers = make_headers(&headers, body.len(), content_type);
        Self {
            version: "HTTP/1.1".to_string(),
            status,
            reason: reason_phrase(status),
            body,
            headers,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes_str = String::new();
        bytes_str.push_str(&self.make_first_line());
        bytes_str.push_str(&self.make_headers());
        bytes_str.push_str("\r\n");

        let mut retval = bytes_str.as_bytes().to_vec();
        retval.extend_from_slice(&self.body);
        retval
    }

    pub fn make_first_line(&self) -> String {
        format!("{} {} {}\r\n", self.version, self.status, self.reason)
    }

    pub fn make_headers(&self) -> String {
        let mut retval = String::new();
        for (key, value) in self.headers.iter() {
            retval.push_str(&format!("{}: {}\r\n", key, value))
        }
        retval
    }
}

fn make_headers(
    headers: &[(String, String)],
    body_len: usize,
    content_type: ContentType,
) -> Vec<(String, String)> {
    let mut retval = Vec::new();

    for header in headers {
        retval.push(header.clone());
    }

    retval.push(("Content-length".to_string(), format!("{}", body_len)));
    retval.push(("Accept-Encoding".to_string(), "".to_string()));
    retval.push(("Connection".to_string(), "Keep-Alive".to_string()));
    retval.push(("Content-Type".to_string(), format!("{}", content_type)));
    retval.push(("Date".to_string(), get_header_date()));
    retval.push(("Server".to_string(), "webserv-rs".to_string()));

    retval
}

fn get_header_date() -> String {
    let now: DateTime<Utc> = Utc::now();
    format!("{}", now.format("%A, %d %m %Y %H:%M:%S GMT"))
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
