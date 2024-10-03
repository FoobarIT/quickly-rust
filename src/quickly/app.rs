use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use crate::quickly::http::{Request, Response}; // On importe les nouveaux types Request et Response
use crate::quickly::router::Router;
use crate::quickly::middleware::Middleware;

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
    
    pub fn get(&mut self, path: &str, handler: fn(&Request, Response) -> Response) {
        self.router.add_route("GET", path, handler);
    }
    pub fn post(&mut self, path: &str, handler: fn(&Request, Response) -> Response) {
        self.router.add_route("POST", path, handler);
    }
    pub fn put(&mut self, path: &str, handler: fn(&Request, Response) -> Response) {
        self.router.add_route("PUT", path, handler);
    }
    pub fn delete(&mut self, path: &str, handler: fn(&Request, Response) -> Response) {
        self.router.add_route("DELETE", path, handler);
    }
    pub fn patch(&mut self, path: &str, handler: fn(&Request, Response) -> Response) {
        self.router.add_route("PATCH", path, handler);
    }
    pub fn options(&mut self, path: &str, handler: fn(&Request, Response) -> Response) {
        self.router.add_route("OPTIONS", path, handler);
    }
    pub fn head(&mut self, path: &str, handler: fn(&Request, Response) -> Response) {
        self.router.add_route("HEAD", path, handler);
    }
    pub fn use_middleware(&mut self, middleware: Middleware) {
        self.middlewares.push(middleware);
    }

    pub fn run(&mut self, port: &str) {
        let addr = format!("127.0.0.1:{}", port);
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
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let request_str = String::from_utf8_lossy(&buffer[..]);

        let request = crate::quickly::http::parse_request(&request_str);

        let response = self.run_middlewares(&request, &self.router);

        let response_str = response.to_string();
        stream.write_all(response_str.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn run_middlewares(&self, req: &Request, router: &Router) -> Response {
        let mut next: Box<dyn Fn(&Request) -> Response> = Box::new(|req: &Request| router.handle_request(req));

        for middleware in self.middlewares.iter().rev() {
            let old_next = next; // Capture l'ancien "next"
            next = Box::new(move |req: &Request| middleware(req, &old_next));
        }

        next(req)
    }
}
