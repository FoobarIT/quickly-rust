use crate::quickly::http::{Request, Response};
pub type Middleware = fn(&mut Request, &dyn Fn(&mut Request) -> Response) -> Response;
