use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub params: HashMap<String, String>, // Params pour stocker les paramètres d'URL
}

pub struct Response {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    // Constructeur de base pour la réponse
    pub fn new(status_code: u16, body: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());

        Response {
            status_code,
            headers,
            body: body.to_string(),
        }
    }

    // Ajouter un header à la réponse
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    // Remplace le corps de la réponse
    pub fn send(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    pub fn json(mut self, body: &str) -> Self {
        self.headers.insert("Content-Type".to_string(), "application/json".to_string());
        self.body = body.to_string();
        self
    }

    pub fn cookie(mut self, key: &str, value: &str, options: &str) -> Self {
        self.headers.insert("Set-Cookie".to_string(), format!("{}={}; {}", key, value, options));
        self
    }
    
    pub fn clear_cookie(mut self, key: &str) -> Self {
        self.headers.insert("Set-Cookie".to_string(), format!("{}=; Max-Age=0", key));
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

impl Request {
    pub fn new(method: &str, path: &str) -> Request {
        Request {
            method: method.to_string(),
            path: path.to_string(),
            headers: HashMap::new(),
            body: String::new(),
            params: HashMap::new(), // Initie les paramètres d'URL
        }
    }
    pub fn param(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }
}

pub fn parse_request(request: &str) -> Result<Request, &str> {
    let lines: Vec<&str> = request.lines().collect();
    let (method, path) = match parse_request_line(lines[0]) {
        Ok((method, path)) => (method, path),
        Err(e) => return Err(e),
    };
    let (headers, body) = parse_headers_and_body(&lines[1..]);

    Ok(Request {
        method: method.to_string(),
        path: path.to_string(),
        headers,
        body,
        params: HashMap::new(),
    })
}

pub fn parse_headers_and_body(lines: &[&str]) -> (HashMap<String, String>, String) {
    let mut headers = HashMap::new();
    let mut body = String::new();
    let mut is_body = false;

    for line in lines {
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
    (headers, body)
}
// Fonction pour analyser la première ligne d'une requête HTTP (méthode et chemin)
fn parse_request_line(request: &str) -> Result<(&str, &str), &str> {
    let parts: Vec<&str> = request.split_whitespace().collect();
    if parts.len() == 3 && parts[2].starts_with("HTTP/") {
        Ok((parts[0], parts[1]))
    } else {
        Err("Invalid request")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_request_line() {
        assert_eq!(parse_request_line("GET /index.html HTTP/1.1"), Ok(("GET", "/index.html")));
        assert_eq!(parse_request_line("INVALID REQUEST"), Err("Invalid request"));
    }

    #[test]
    fn test_parse_header() {
        assert_eq!(parse_header("Host: localhost"), Some(("Host".to_string(), "localhost".to_string())));
        assert_eq!(parse_header("Invalid Header"), None);
    }

    #[test]
    fn test_response_json() {
        let response = Response::new(200, "").json(r#"{"key": "value"}"#);
        
        assert_eq!(response.headers.get("Content-Type"), Some(&"application/json".to_string()));
        assert_eq!(response.body, r#"{"key": "value"}"#);
    }

    #[test]
    fn test_response_header() {
        let response = Response::new(200, "")
            .header("X-Custom-Header", "Value");
        assert_eq!(response.headers.get("X-Custom-Header"), Some(&"Value".to_string()));
    }

    #[test]
    fn test_response_send() {
        let response = Response::new(200, "")
            .send("body");
        assert_eq!(response.body, "body");
    }
    #[test]
    fn test_parse_headers_and_body() {
        let (headers, body) = parse_headers_and_body(&[
            "Host: localhost",
            "Content-Length: 5",
            "",
            "hello",
        ]);

        assert_eq!(headers.get("Host"), Some(&"localhost".to_string()));
        assert_eq!(headers.get("Content-Length"), Some(&"5".to_string()));
        assert_eq!(body, "hello");
    }
}
