pub type Middleware = fn(&str, &dyn Fn(&str) -> crate::response::Response) -> crate::response::Response;
