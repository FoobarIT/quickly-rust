use std::collections::HashMap;

type Handler = fn(&str) -> crate::response::Response;

pub struct Router {
    routes: HashMap<String, Handler>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, method: &str, path: &str, handler: Handler) {
        let route_key = format!("{} {}", method, path);
        self.routes.insert(route_key, handler);
    }

    pub fn handle_request(&self, request: &str) -> crate::response::Response {
        let (method, path) = parse_request(request);
        let route_key = format!("{} {}", method, path);

        match self.routes.get(&route_key) {
            Some(handler) => handler(request),
            None => crate::response::Response::new(404, "Not Found"),
        }
    }
}

fn parse_request(request: &str) -> (&str, &str) {
    let lines: Vec<&str> = request.lines().collect();
    if !lines.is_empty() {
        let parts: Vec<&str> = lines[0].split_whitespace().collect();
        if parts.len() == 3 {
            return (parts[0], parts[1]);
        }
    }
    ("GET", "/")
}
