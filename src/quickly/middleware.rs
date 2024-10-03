use crate::quickly::http::{Request, Response};
pub type Middleware = fn(&Request, &dyn Fn(&Request) -> Response) -> Response;