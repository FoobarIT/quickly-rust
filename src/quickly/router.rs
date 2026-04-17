use crate::quickly::http::{Request, Response};
use std::collections::HashMap;

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
        let mut allowed_methods = Vec::new();

        for (route_key, handler) in &self.routes {
            let (route_method, route_path) = self.parse_route_key(route_key);

            if let Some(params) = self.match_path(route_path, path) {
                if method == route_method {
                    request.params = params;
                    found_route = Some(handler);
                    break;
                } else {
                    allowed_methods.push(route_method);
                }
            }
        }

        match found_route {
            Some(handler) => handler(request, Response::new(200, "OK")),
            None if !allowed_methods.is_empty() => {
                allowed_methods.sort_unstable();
                allowed_methods.dedup();

                Response::new(405, "Method Not Allowed")
                    .header("Allow", &allowed_methods.join(", "))
            }
            None => Response::new(404, "Not Found"),
        }
    }

    fn parse_route_key<'a>(&'a self, route_key: &'a str) -> (&'a str, &'a str) {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn ok_handler(_req: &mut Request, res: Response) -> Response {
        res.send("ok")
    }

    #[test]
    fn test_handle_request_returns_method_not_allowed_for_known_path() {
        let mut router = Router::new();
        router.get_test_route_setup();

        let mut request = Request::new("DELETE", "/users/42");
        let response = router.handle_request(&mut request);

        assert_eq!(response.status_code, 405);
        assert_eq!(
            response.headers.get("Allow"),
            Some(&"GET, POST".to_string())
        );
    }

    #[test]
    fn test_handle_request_returns_not_found_for_unknown_path() {
        let mut router = Router::new();
        router.get_test_route_setup();

        let mut request = Request::new("GET", "/missing");
        let response = router.handle_request(&mut request);

        assert_eq!(response.status_code, 404);
        assert!(!response.headers.contains_key("Allow"));
    }

    impl Router {
        fn get_test_route_setup(&mut self) {
            self.add_route("POST", "/users/:id", ok_handler);
            self.add_route("GET", "/users/:id", ok_handler);
        }
    }
}
