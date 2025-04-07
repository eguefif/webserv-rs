use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Request {
    method: String,
    uri: String,
    version: String,
    headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn new(response: &str) -> Self {
        Self {
            method: get_method(&response),
            uri: get_uri(&response),
            version: get_version(&response),
            headers: get_headers(&response),
            body: Vec::new(),
        }
    }

    pub fn get_value(&self, key: &str) -> Option<&str> {
        for (header_key, value) in self.headers.iter() {
            if header_key == key {
                return Some(value);
            }
        }
        None
    }

    pub fn is_chunked(&self) -> bool {
        if let Some(te) = self.get_value("Transfer-Encoding") {
            if te.to_lowercase().trim() == "chunked" {
                return true;
            }
            if let Some((_, last)) = te.rsplit_once(",") {
                if last.trim().to_lowercase() == "chunked" {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_body(&self) -> bool {
        if let Some(_) = self.get_value("Content-Length") {
            return true;
        }
        if let Some(_) = self.get_value("Transfer-Encoding") {
            return true;
        }
        false
    }

    pub fn get_content_length(&self) -> Option<usize> {
        if let Some(length) = self.get_value("Content-Length") {
            if let Ok(length) = length.parse::<usize>() {
                return Some(length);
            } else {
                return None;
            }
        }
        None
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        format!("{}", self).as_bytes().to_vec()
    }
}

fn get_method(response: &str) -> String {
    let mut splits = response.split_ascii_whitespace();
    splits.next().unwrap().to_string()
}

fn get_uri(response: &str) -> String {
    let mut splits = response.split_ascii_whitespace();
    splits.next().unwrap();
    splits.next().unwrap().trim().to_string()
}

fn get_version(response: &str) -> String {
    let mut splits = response.split_ascii_whitespace();
    splits.next().unwrap();
    splits.next().unwrap();
    splits.next().unwrap().trim().to_string()
}

fn get_headers(response: &str) -> Vec<(String, String)> {
    let mut headers: Vec<(String, String)> = Vec::new();
    let mut lines = response.lines();
    lines.next().unwrap();
    for line in lines {
        if line.contains(":") {
            if let Some((key, value)) = line.split_once(":") {
                headers.push((key.trim().to_string(), value.trim().to_string()));
            }
        }
    }
    headers
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut request = String::new();
        request.push_str(&format!(
            "{} {} {}\r\n",
            self.method, self.uri, self.version
        ));
        for (key, value) in self.headers.iter() {
            request.push_str(&format!("{}: {}\r\n", key, value));
        }
        if self.body.len() > 0 {
            request.push_str(&format!("\r\n{}", String::from_utf8_lossy(&self.body)));
        }
        write!(f, "{}", request)
    }
}
