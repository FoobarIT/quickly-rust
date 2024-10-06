use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use crate::quickly::http::{Request, Response}; // On importe les nouveaux types Request et Response
use crate::quickly::router::Router;

const ADDR: &str = "127.0.0.1";
const BUFFER_SIZE: usize = 1024;


struct Middleware {
    path: Option<String>,
    func: Box<dyn Fn(&mut Request, &dyn Fn(&mut Request) -> Response) -> Response>,
}

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
        let addr = format!("{}:{}",ADDR, port);
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
            let old_next = next;
            let path = middleware.path.clone();
            next = Box::new(move |req: &mut Request| {
                if let Some(ref p) = path {
                    if req.path.starts_with(p) {
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