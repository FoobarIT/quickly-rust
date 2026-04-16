use crate::quickly::http::{Request, Response};
use crate::quickly::router::Router;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

const ADDR: &str = "127.0.0.1";
const READ_BUFFER_SIZE: usize = 1024;
const MAX_REQUEST_SIZE: usize = 64 * 1024;
const STREAM_READ_TIMEOUT: Duration = Duration::from_secs(5);

struct Middleware {
    path: Option<String>,
    func: Box<dyn Fn(&mut Request, &dyn Fn(&mut Request) -> Response) -> Response>,
}

pub struct App {
    router: Router,
    middlewares: Vec<Middleware>,
}

enum ReadHttpRequestError {
    Io(std::io::Error),
    Timeout,
    RequestTooLarge,
}

impl App {
    pub fn new() -> App {
        App {
            router: Router::new(),
            middlewares: Vec::new(),
        }
    }

    pub fn get(&mut self, path: &str, handler: fn(&mut Request, Response) -> Response) {
        self.router.add_route("GET", path, handler);
    }
    pub fn post(&mut self, path: &str, handler: fn(&mut Request, Response) -> Response) {
        self.router.add_route("POST", path, handler);
    }
    pub fn put(&mut self, path: &str, handler: fn(&mut Request, Response) -> Response) {
        self.router.add_route("PUT", path, handler);
    }
    pub fn delete(&mut self, path: &str, handler: fn(&mut Request, Response) -> Response) {
        self.router.add_route("DELETE", path, handler);
    }
    pub fn patch(&mut self, path: &str, handler: fn(&mut Request, Response) -> Response) {
        self.router.add_route("PATCH", path, handler);
    }
    pub fn options(&mut self, path: &str, handler: fn(&mut Request, Response) -> Response) {
        self.router.add_route("OPTIONS", path, handler);
    }
    pub fn head(&mut self, path: &str, handler: fn(&mut Request, Response) -> Response) {
        self.router.add_route("HEAD", path, handler);
    }
    pub fn method(
        &mut self,
        method: &str,
        path: &str,
        handler: fn(&mut Request, Response) -> Response,
    ) {
        self.router.add_route(method, path, handler);
    }

    pub fn work<F>(&mut self, path: Option<&str>, func: F)
    where
        F: Fn(&mut Request, &dyn Fn(&mut Request) -> Response) -> Response + 'static,
    {
        self.middlewares.push(Middleware {
            path: path.map(|p| p.to_string()),
            func: Box::new(func),
        });
    }

    pub fn run(&mut self, port: &str) {
        let addr = format!("{}:{}", ADDR, port);
        println!("Listening on http://{}", addr);

        let listener = TcpListener::bind(&addr).expect("Failed to bind to address");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_connection(stream);
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
    }

    fn read_http_request(&self, stream: &mut TcpStream) -> std::io::Result<String> {
        let mut buffer = [0; READ_BUFFER_SIZE];
        let mut request_data = Vec::new();

        loop {
            let bytes_read = stream.read(&mut buffer)?;

            if bytes_read == 0 {
                break;
            }

            request_data.extend_from_slice(&buffer[..bytes_read]);

            if request_data.len() > MAX_REQUEST_SIZE {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Request exceeds maximum size",
                ));
            }

            let request_str = String::from_utf8_lossy(&request_data);

            if let Some(headers_end) = request_str.find("\r\n\r\n") {
                let headers = &request_str[..headers_end];
                let body_start = headers_end + 4;

                let content_length = headers
                    .lines()
                    .find_map(|line| {
                        let mut parts = line.splitn(2, ':');
                        let key = parts.next()?.trim();
                        let value = parts.next()?.trim();

                        if key.eq_ignore_ascii_case("Content-Length") {
                            value.parse::<usize>().ok()
                        } else {
                            None
                        }
                    })
                    .unwrap_or(0);

                if body_start + content_length > MAX_REQUEST_SIZE {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Request exceeds maximum size",
                    ));
                }

                if request_data.len() >= body_start + content_length {
                    break;
                }
            }
        }

        Ok(String::from_utf8_lossy(&request_data).to_string())
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        if let Err(e) = stream.set_read_timeout(Some(STREAM_READ_TIMEOUT)) {
            eprintln!("Failed to configure read timeout: {}", e);
        }

        let read_result = self
            .read_http_request(&mut stream)
            .map_err(|err| match err.kind() {
                std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock => {
                    ReadHttpRequestError::Timeout
                }
                std::io::ErrorKind::InvalidData
                    if err.to_string() == "Request exceeds maximum size" =>
                {
                    ReadHttpRequestError::RequestTooLarge
                }
                _ => ReadHttpRequestError::Io(err),
            });

        match read_result {
            Ok(request_str) => {
                let request_result = crate::quickly::http::parse_request(&request_str);

                let response = match request_result {
                    Ok(mut request) => self.run_middlewares(&mut request, &self.router),
                    Err(err) => {
                        eprintln!("Failed to parse request: {}", err);
                        crate::quickly::http::Response::new(400, "Bad Request")
                    }
                };

                let response_str = response.to_string();
                stream.write_all(response_str.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Err(ReadHttpRequestError::Timeout) => {
                let response = Response::new(408, "Request Timeout");
                let response_str = response.to_string();
                if let Err(e) = stream.write_all(response_str.as_bytes()) {
                    eprintln!("Failed to write timeout response: {}", e);
                }
            }
            Err(ReadHttpRequestError::RequestTooLarge) => {
                let response = Response::new(413, "Payload Too Large");
                let response_str = response.to_string();
                if let Err(e) = stream.write_all(response_str.as_bytes()) {
                    eprintln!("Failed to write size limit response: {}", e);
                }
            }
            Err(ReadHttpRequestError::Io(e)) => {
                eprintln!("Failed to read stream: {}", e);
            }
        }
    }

    fn run_middlewares(&self, req: &mut Request, router: &Router) -> Response {
        let mut next: Box<dyn Fn(&mut Request) -> Response> =
            Box::new(|req: &mut Request| router.handle_request(req));

        for middleware in self.middlewares.iter().rev() {
            let old_next = next;
            let path = middleware.path.clone();
            next = Box::new(move |req: &mut Request| {
                if let Some(ref p) = path {
                    if path_matches_middleware(req.path.as_str(), p.as_str()) {
                        return (middleware.func)(req, &old_next);
                    }
                } else {
                    return (middleware.func)(req, &old_next);
                }
                old_next(req)
            });
        }
        next(req)
    }
}

fn path_matches_middleware(request_path: &str, middleware_path: &str) -> bool {
    request_path == middleware_path
        || request_path
            .strip_prefix(middleware_path)
            .is_some_and(|suffix| suffix.starts_with('/'))
}

#[cfg(test)]
mod tests {
    use super::path_matches_middleware;

    #[test]
    fn test_path_matches_middleware_exact_match() {
        assert!(path_matches_middleware("/json", "/json"));
    }

    #[test]
    fn test_path_matches_middleware_nested_path() {
        assert!(path_matches_middleware("/json/items", "/json"));
    }

    #[test]
    fn test_path_matches_middleware_rejects_partial_prefix() {
        assert!(!path_matches_middleware("/jsonx", "/json"));
    }
}
