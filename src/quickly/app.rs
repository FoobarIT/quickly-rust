use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use crate::quickly::http::{Request, Response}; // On importe les nouveaux types Request et Response
use crate::quickly::router::Router;
use crate::quickly::middleware::Middleware;


const ADDR: &str = "127.0.0.1";
const BUFFER_SIZE: usize = 1024;
pub struct App {
    router: Router,
    middlewares: Vec<Middleware>,
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
    pub fn use_middleware(&mut self, middleware: Middleware) {
        self.middlewares.push(middleware);
    }

    pub fn run(&mut self, port: &str) {
        let addr = format!("{} {}",ADDR, port);
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

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; BUFFER_SIZE];
        match stream.read(&mut buffer) {
            Ok(_) => {
                let request_str = String::from_utf8_lossy(&buffer[..]);

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
            Err(e) => {
                eprintln!("Failed to read stream: {}", e);
            }
        }
    }

    fn run_middlewares(&self, req: &mut Request, router: &Router) -> Response {
        let mut next: Box<dyn Fn(&mut Request) -> Response> = Box::new(|req: &mut Request| router.handle_request(req));

        for middleware in self.middlewares.iter().rev() {
            let old_next = next; // Capture l'ancien "next"
            next = Box::new(move |req: &mut Request| middleware(req, &old_next));
        }

        // Exécution du premier middleware ou de la route
        next(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_handler(_req: &mut Request, _res: Response) -> Response {
        Response::new(200, "Ok")
    }

    fn test_route(app: &mut App, method: &str, path: &str) {
        match method {
            "GET" => app.get(path, mock_handler),
            "POST" => app.post(path, mock_handler),
            "PUT" => app.put(path, mock_handler),
            "DELETE" => app.delete(path, mock_handler),
            "PATCH" => app.patch(path, mock_handler),
            "OPTIONS" => app.options(path, mock_handler),
            "HEAD" => app.head(path, mock_handler),
            _ => panic!("Unsupported method"),
        }
        app.router.add_route(method, path, mock_handler);
    }

    #[test]
    fn test_get_route() {
        let mut app = App::new();
        test_route(&mut app, "GET", "/test");
    }

    #[test]
    fn test_post_route() {
        let mut app = App::new();
        test_route(&mut app, "POST", "/test");
    }

    #[test]
    fn test_put_route() {
        let mut app = App::new();
        test_route(&mut app, "PUT", "/test");
    }

    #[test]
    fn test_delete_route() {
        let mut app = App::new();
        test_route(&mut app, "DELETE", "/test");
    }

    #[test]
    fn test_patch_route() {
        let mut app = App::new();
        test_route(&mut app, "PATCH", "/test");
    }

    #[test]
    fn test_options_route() {
        let mut app = App::new();
        test_route(&mut app, "OPTIONS", "/test");
    }

    #[test]
    fn test_head_route() {
        let mut app = App::new();
        test_route(&mut app, "HEAD", "/test");
    }

    #[test]
    fn test_use_middleware() {
        let mut app = App::new();
        let middleware: Middleware = |req, next| next(req);
        app.use_middleware(middleware);
        assert_eq!(app.middlewares.len(), 1);
    }
}