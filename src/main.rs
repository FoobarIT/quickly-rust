use quickly_rust::quickly::app::App;
use quickly_rust::quickly::http::{Request, Response};

fn main() {
    let mut app = App::new();

    app.get("/", |_req: &Request, mut res: Response| {
        res.body = String::from("Hello, World:");
        res
    });
 

    app.use_middleware(|req, next| {
        println!("Middleware: Before");
        let response = next(req);
        println!("Middleware: After");
        response
    });

    app.run("3000");
}
