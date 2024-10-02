pub struct Response {
    status_code: u16,
    body: String,
}

impl Response {
    pub fn new(status_code: u16, body: &str) -> Response {
        Response {
            status_code,
            body: body.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("HTTP/1.1 {} OK\r\n\r\n{}", self.status_code, self.body)
    }
}
