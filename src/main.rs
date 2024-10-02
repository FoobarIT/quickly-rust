mod app;
mod router;
mod middleware;
mod response;

use app::App;

fn main() {
    let mut app = App::new();

    app.get("/", |_req| {
        response::Response::new(200, "Hello, World!")
    });

    app.use_middleware(|req, next| {
        println!("Middleware: Before");
        let response = next(req);
        println!("Middleware: After");
        response
    });

    app.run("3000");
}
