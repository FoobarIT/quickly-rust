# Quickly Rust v0.01

This project implements a small web server in Rust, capable of handling HTTP requests with multiple methods (GET, POST, PUT, DELETE, etc.) and middleware management. The server uses a custom router to match routes with specific handlers and supports URL parameters.

*Please dont using it on production, wait v1.00! 🤡*


## Table of Contents

- [Project Structure](#project-structure)
- [Main Components](#main-components)
  - [App](#app)
  - [Request](#request)
  - [Response](#response)
  - [Middleware](#middleware)
- [Usage Example](#usage-example)
- [Running the Server](#running-the-server)
- [Contributing](#contribute)
- [License](#license) 

## Project Structure

- `main.rs`: Main file where the application is started and routes are defined.
- `app.rs`: Contains the main application structure (`App`) and manages requests as well as middleware execution.
- `router.rs`: Manages route registration and matches incoming requests with the defined routes.
- `http.rs`: Defines the `Request` and `Response` structs and the function to parse a raw HTTP request.
- `middleware.rs`: Defines middleware, which allows intercepting and modifying requests before they are processed by the router.

## Main Components

### App

The `App` component is the central element of the application. It manages the router, middleware, and the logic for handling TCP connections.

#### Key Methods

- `new()`: Creates a new instance of the application.
- `get(path, handler)`: Adds a GET route to the router.
- `post(path, handler)`: Adds a POST route to the router.
- `run(port)`: Starts the TCP server on the specified port and processes incoming requests.
- `use_middleware(middleware)`: Adds a middleware that will be applied to every request.

#### Example

```rust
let mut app = App::new();
app.get("/", |_req, res| {        
    res.send("Hello, world!")
});
app.run("3000");
```

### Request
Request is the structure that contains information about an incoming HTTP request.

#### Fields
* `method`: The HTTP method (GET, POST, etc.).
* `path`: The request path.
* `headers`: The request headers.
* `body`: The request body.
* `params`: The dynamic URL parameters.
#### Key Methods
* `param(key)`: Retrieves the value of a URL parameter.
#### Example
```rust
let method = req.method;
let user_id = req.param("id");
```

### Response
Response contains the information needed to send an HTTP response.

#### Fields
* `status_code`: The HTTP status code (e.g., 200 for OK).
* `headers`: The response headers.
* `body`: The response body.
#### Key Methods
* `new(status_code, body)`: Creates a new response with a status code and body.
* `header(key, value)`: Adds a header to the response.
* `send(body)`: Sets the response body.
* `to_string()`: Formats the response for sending over a TCP connection.
#### Example
```rust
res.send("Hello, world!") 
```

### Middleware
Middlewares are functions that intercept and modify requests before they reach route handlers.

#### Signature
```rust
type Middleware = fn(&mut Request, &dyn Fn(&mut Request) -> Response) -> Response;
```
#### Example
```rust
app.use_middleware(|req: &mut Request, next| {
    println!("Middleware: request method {:?}", req.method);
    next(req)
});
```

## Usage Example
```rust
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
```

## Running the Server
To start the server, compile and run the project using Cargo:

```bash
cargo run
```
The server will be accessible at http://127.0.0.1:3000.

## CONTRIBUTE 
[CONTRIBUTING](CONTRIBUTING.md)

## LICENSE
[MIT](LICENSE)