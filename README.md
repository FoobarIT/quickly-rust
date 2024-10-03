## QUICKLY RUST
quickly-rust is a framework project with the aim of learning rust, any contribution or modification is acceptable as long as you correctly explain the modification or addition you wish to make. (Should be like express.js principe)


## BASIC USAGE
```rust
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
```