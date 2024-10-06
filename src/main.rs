use quickly_rust::quickly::app::App;

fn main() {
    let mut app = App::new();

    app.get("/", |_req, res| {
        res.send("Hello, world!")
    });
    
    app.get("/json", |_req, res| {
        res.json(r#"{"message": "Hello, world!"}"#)
    });

    app.get("/users/:id", |req, res| {
        if let Some(user_id) = req.param("id") {
            res.send(&format!("User ID: {}", user_id))
        } else {
            res.send("Bad ID")
        }
    });

    // All routes apply to this middleware
    app.work(None, |req, next| {
        println!("Middleware: Global");
        println!("Middleware: Before");
        let response = next(req);
        println!("Middleware: After");
        response
    });

    app.work(Some("/json"), |req, next| {
        println!("Middleware: JSON");
        let response = next(req);
        response
    });

    app.run("3000");
    
}

