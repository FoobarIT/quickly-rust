use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

pub struct Response {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn new(status_code: u16, body: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());

        Response {
            status_code,
            headers,
            body: body.to_string(),
        }
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn send(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    pub fn to_string(&self) -> String {
        let mut response = format!("HTTP/1.1 {} OK\r\n", self.status_code);

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str("\r\n");
        response.push_str(&self.body);

        response
    }
}

// La fonction parse_request convertit une requête brute en un objet Request structuré
pub fn parse_request(request: &str) -> Request {
    let lines: Vec<&str> = request.lines().collect();
    let (method, path) = parse_request_line(lines[0]);

    // Extraire les headers
    let mut headers = HashMap::new();
    let mut body = String::new();
    let mut is_body = false;

    for line in &lines[1..] {
        if line.is_empty() {
            is_body = true;
            continue;
        }
        if is_body {
            body.push_str(line);
        } else {
            if let Some((key, value)) = parse_header(line) {
                headers.insert(key, value);
            }
        }
    }

    Request {
        method: method.to_string(),
        path: path.to_string(),
        headers,
        body,
    }
}

fn parse_request_line(line: &str) -> (&str, &str) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 2 {
        (parts[0], parts[1])
    } else {
        ("GET", "/")
    }
}

fn parse_header(line: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = line.splitn(2, ':').collect();
    if parts.len() == 2 {
        Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
    } else {
        None
    }
}
