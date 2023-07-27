use std::collections::HashMap;

#[derive(Debug)]
pub struct Response {
    pub http_version: String,
    pub status_code: u16,
    pub reason_phrase: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<Vec<u8>>,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            http_version: "HTTP/1.1".to_string(),
            status_code: 200,
            reason_phrase: "OK".to_string(),
            headers: None,
            body: None,
        }
    }
}
pub struct ResponseBuilder {
    pub response: Response,
}

impl ResponseBuilder {
    pub fn new() -> ResponseBuilder {
        ResponseBuilder {
            response: Response::default(),
        }
    }
    pub fn http_version(mut self, http_version: String) -> Self {
        self.response.http_version = http_version;
        self
    }

    pub fn status_code(mut self, status_code: u16) -> Self {
        self.response.status_code = status_code;
        self
    }

    pub fn reason_phrase(mut self, reason_phrase: String) -> Self {
        self.response.reason_phrase = reason_phrase;
        self
    }

    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.response.headers = Some(headers);
        self
    }

    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.response.body = Some(body);
        self
    }

    pub fn build(self) -> Response {
        self.response
    }
}
