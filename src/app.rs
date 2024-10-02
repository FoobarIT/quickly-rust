use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use crate::router::Router;
use crate::middleware::Middleware;

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

    pub fn get(&mut self, path: &str, handler: fn(&str) -> crate::response::Response) {
        self.router.add_route("GET", path, handler);
    }

    pub fn use_middleware(&mut self, middleware: Middleware) {
        self.middlewares.push(middleware);
    }

    pub fn run(&mut self, port: &str) {
        let default_addr: &str = "127.0.0.1:";
        let addr = default_addr.to_string() + port;
        println!("{}", addr);
        let listener = TcpListener::bind(addr).expect("Failed to bind to address");
        println!("Listening on http://127.0.0.1:{}", port);

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
        let request = String::from_utf8_lossy(&buffer[..]);

        // Gestion du middleware
        let response = self.run_middlewares(&request, &self.router);

        // Envoi de la réponse
        let response_str = response.to_string();
        stream.write_all(response_str.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn run_middlewares(&self, req: &str, router: &Router) -> crate::response::Response {
        // Boxe la première closure
        let mut next: Box<dyn Fn(&str) -> crate::response::Response> =
            Box::new(|req: &str| router.handle_request(req));

        for middleware in self.middlewares.iter().rev() {
            let old_next = next; // Capture l'ancien "next"
            next = Box::new(move |req: &str| middleware(req, &old_next));
        }

        next(req)
    }
}
