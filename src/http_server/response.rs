use crate::http_server::{ContentType, Cookie};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Response {
    pub http_version: String,
    pub status_code: u16,
    pub reason_phrase: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<Vec<u8>>,
    pub location: Option<String>,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            http_version: "HTTP/1.1".to_string(),
            status_code: 200,
            reason_phrase: "OK".to_string(),
            headers: None,
            body: None,
            location: None,
        }
    }
}
pub struct ResponseBuilder {
    pub response: Response,
}

impl ResponseBuilder {
    pub fn content_type(mut self, content_type: ContentType) -> Self {
        if self.response.headers.is_none() {
            self.response.headers = Some(HashMap::new());
        }
        self.response
            .headers
            .as_mut()
            .unwrap()
            .insert("Content-Type".to_string(), content_type.to_string());
        self
    }
    pub fn temp_redirect(mut self, location: impl ToString) -> Self {
        if self.response.headers.is_none() {
            self.response.headers = Some(HashMap::new());
        }
        self.response
            .headers
            .as_mut()
            .unwrap()
            .insert("Location".to_string(), location.to_string());
        self = self.status_code(302);
        self
    }
    pub fn body_string(mut self, body: String) -> Self {
        let body_bytes = body.into_bytes();
        let body_len = body_bytes.len();
        self.response.body = Some(body_bytes);
        self = self.content_length(body_len);
        self
    }
    fn content_length(mut self, content_length: usize) -> Self {
        if self.response.headers.is_none() {
            self.response.headers = Some(HashMap::new());
        }
        self.response
            .headers
            .as_mut()
            .unwrap()
            .insert("Content-Length".to_string(), content_length.to_string());
        self
    }

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
    pub fn body_bytes(mut self, body: Vec<u8>) -> Self {
        let content_length = body.len();
        self.response.body = Some(body);
        self = self.content_length(content_length);
        self
    }
    pub fn cookie(mut self, cookie: Cookie) -> Self {
        if self.response.headers.is_none() {
            self.response.headers = Some(HashMap::new());
        }

        if let Some(map) = self.response.headers.as_mut() {
            map.insert("Set-Cookie".to_string(), cookie.cookie_string());
        }

        self
    }

    pub fn build(self) -> Response {
        self.response
    }
}
