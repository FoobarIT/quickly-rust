use std::collections::HashMap;
use crate::quickly::http::{Request, Response};

// Un Handler prend un `Request` et retourne un `Response`
type Handler = fn(&Request, Response) -> Response;

pub struct Router {
    routes: HashMap<String, Handler>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    // Ajoute une route au routeur avec la méthode HTTP et le chemin
    pub fn add_route(&mut self, method: &str, path: &str, handler: Handler) {
        let route_key = format!("{} {}", method, path);
        self.routes.insert(route_key, handler);
    }

    // Gère une requête et retourne une réponse
    pub fn handle_request(&self, request: &Request) -> Response {
        let route_key = format!("{} {}", request.method, request.path);
        //TODO Check for good way here.
        let response = Response::new(200, "");
        // Cherche le handler correspondant à la route
        match self.routes.get(&route_key) {
            Some(handler) => handler(request, response),
            None => Response::new(404, "Not Found"),
        }
    }
}
