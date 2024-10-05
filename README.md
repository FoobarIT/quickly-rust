## QUICKLY RUST *v0.01*
quickly-rust is a framework project with the aim of learning rust, any contribution or modification is acceptable as long as you correctly explain the modification or addition you wish to make. (Should be like express.js principe)

*Please dont use it on prod*


## BASIC USAGE
```rust
use quickly_rust::quickly::app::App;

fn main() {
    let mut app = App::new();

    app.get("/", |_req, res| {
        res.send("New page")
    });
    //Test route pour le stream:
    app.get("/users/:id", |req, res| {
        if let Some(user_id) = req.param("id") {
            res.send(&format!("User ID: {}", user_id))
        } else {
            res.send("Mauvais ID")
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
```


