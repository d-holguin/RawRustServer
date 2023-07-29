use std::collections::HashMap;

#[derive(Debug, Clone)]
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

pub enum ContentType {
    Html,
    PlainTest,
    Json,
    Ico,
}

impl ContentType {
    fn as_str(&self) -> &'static str {
        match *self {
            ContentType::Html => "text/html",
            ContentType::Json => "application/json",
            ContentType::PlainTest => "text/plain",
            ContentType::Ico => "image/x-icon",
        }
    }
}
impl ToString for ContentType {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
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
    pub fn build(self) -> Response {
        self.response
    }
}
