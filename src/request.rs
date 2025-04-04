use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Request {
    method: String,
    uri: String,
    version: String,
    headers: Vec<(String, String)>,
}

impl Request {
    pub fn new(response: &str) -> Self {
        Self {
            method: get_method(&response),
            uri: get_uri(&response),
            version: get_version(&response),
            headers: get_headers(&response),
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
}

fn get_method(response: &str) -> String {
    let mut splits = response.split_ascii_whitespace();
    splits.next().unwrap().to_string()
}

fn get_uri(response: &str) -> String {
    let mut splits = response.split_ascii_whitespace();
    splits.next().unwrap();
    splits.next().unwrap().to_string()
}

fn get_version(response: &str) -> String {
    let mut splits = response.split_ascii_whitespace();
    splits.next().unwrap();
    splits.next().unwrap();
    splits.next().unwrap().to_string()
}

fn get_headers(response: &str) -> Vec<(String, String)> {
    let mut headers: Vec<(String, String)> = Vec::new();
    let mut lines = response.lines();
    lines.next().unwrap();
    for line in lines {
        if line.contains(":") {
            let mut splits = line.split(":");
            let key = splits.next().unwrap().trim().to_string();
            let value = splits.next().unwrap().trim().to_string();
            headers.push((key, value))
        }
    }
    headers
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut header = String::new();
        header.push_str(&format!("{} {} {}\n", self.method, self.uri, self.version));
        for (key, value) in self.headers.iter() {
            header.push_str(&format!("{}: {}\n", key, value));
        }
        write!(f, "{}", header)
    }
}
