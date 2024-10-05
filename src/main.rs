use quickly_rust::quickly::app::App;

fn main() {
    let mut app = App::new();

    app.get("/", |_req, res| {
        res.send("Hello, world!")
    });
    
    app.get("/users/:id", |req, res| {
        if let Some(user_id) = req.param("id") {
            res.send(&format!("User ID: {}", user_id))
        } else {
            res.send("Bad ID")
        }
    });

    app.use_middleware(|req, next| {
        println!("Middleware: Before");
        let response = next(req);
        println!("Middleware: After");
        response
    });

    app.run("3000");
    
}

