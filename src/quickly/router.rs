use std::collections::HashMap;
use crate::quickly::http::{Request, Response};

type Handler = fn(&mut Request, Response) -> Response;

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

    pub fn handle_request(&self, request: &mut Request) -> Response {
        let (method, path) = (request.method.as_str(), request.path.as_str());
        let mut found_route = None;

        for (route_key, handler) in &self.routes {
            let (route_method, route_path) = self.parse_route_key(route_key);

            if method == route_method {
                if let Some(params) = self.match_path(route_path, path) {
                    request.params = params;
                    found_route = Some(handler);
                    break;
                }
            }
        }

        match found_route {
            Some(handler) => handler(request, Response::new(200, "OK")),
            None => Response::new(404, "Not Found")
        }
    }

    fn parse_route_key<'a>(&'a self, route_key: &'a str) -> (&str, &str) {
        let parts: Vec<&str> = route_key.split_whitespace().collect();
        (parts[0], parts[1])
    }

    fn match_path(&self, route_path: &str, request_path: &str) -> Option<HashMap<String, String>> {
        let route_segments: Vec<&str> = route_path.split('/').collect();
        let request_segments: Vec<&str> = request_path.split('/').collect();

        if route_segments.len() != request_segments.len() {
            return None;
        }

        let mut params = HashMap::new();
        for (route_segment, request_segment) in route_segments.iter().zip(request_segments.iter()) {
            if route_segment.starts_with(':') {
                let key = &route_segment[1..];
                params.insert(key.to_string(), request_segment.to_string());
            } else if route_segment != request_segment {
                return None;
            }
        }
        Some(params)
    }
}
