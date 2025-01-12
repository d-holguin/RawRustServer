
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Lines},
    net::TcpStream,
    str::FromStr,
};

use crate::error::Result;
use crate::http_server::{ContentType, Cookie, Route};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    GET,
    POST,
}
impl FromStr for HttpMethod {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            _ => Err(format!("Invalid Http method {}", s)),
        }
    }
}

impl HttpMethod {
    pub fn get(path: &str) -> Route {
        Route::new()
            .http_method(HttpMethod::GET)
            .path(path.to_string())
    }
    pub fn post(path: &str) -> Route {
        Route::new()
            .http_method(HttpMethod::POST)
            .path(path.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn from_reader(reader: &mut BufReader<TcpStream>) -> Result<Request> {
        let mut lines = reader.lines();
        let request_line = read_request_line(&mut lines)?;
        let headers = parse_headers(&mut lines)?;
        // Read body
        let mut body = Vec::new();

        if let Some(len) = headers.get("content-length") {
            let len: usize = len
                .parse::<usize>()
                .map_err(|_| "Error parsing content length".to_string())?;
            body.reserve(len);
            while body.len() < len {
                let buffer = reader
                    .fill_buf()
                    .map_err(|_| "Error reading request body".to_string())?;
                let bytes_to_read = std::cmp::min(buffer.len(), len - body.len());
                body.extend_from_slice(&buffer[..bytes_to_read]);
                reader.consume(bytes_to_read);
            }
        }

        // parse request_line for method, path and http_version
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        let (method_string, path, http_version) = (
            parts[0].to_string(),
            parts[1].to_string(),
            parts[2].to_string(),
        );

        let method = HttpMethod::from_str(&method_string)
            .map_err(|_| "Invalid HTTP Method".to_string())?;

        Ok(Request {
            method,
            path,
            http_version,
            headers,
            body,
        })
    }

    pub fn form_urlencoded(&self) -> Option<HashMap<String, String>> {
        match self.content_type() {
            Some(ContentType::FormUrlEncoded) => {
                String::from_utf8(self.body.clone()).ok().map(|body_str| {
                    body_str
                        .split('&')
                        .filter_map(|part| {
                            let mut split = part.splitn(2, '=');
                            Some((split.next()?.to_string(), split.next()?.to_string()))
                        })
                        .collect::<HashMap<String, String>>()
                })
            }
            _ => None,
        }
    }
    pub fn cookies(&self) -> Vec<Cookie> {
        let mut cookies = Vec::new();
        if let Some(cookie_header) = self.headers.get("cookie") {
            for cookie_str in cookie_header.split(';') {
                let parts: Vec<&str> = cookie_str.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let name = parts[0].trim().to_string();
                    let value = parts[1].trim().to_string();
                    cookies.push(Cookie::new(name, value));
                }
            }
        }
        cookies
    }

    pub fn content_type(&self) -> Option<ContentType> {
        self.headers
            .get("content-type")
            .and_then(|s| ContentType::from_str(s).ok())
    }
}

fn read_request_line(lines: &mut Lines<&mut BufReader<TcpStream>>) -> Result<String> {
    let request_line_result = lines.next().ok_or_else(|| "No request line")?;
    let request_line_str = request_line_result
        .map_err(|error| format!("Unable to get request string: {}", error))?;
    let parts: Vec<&str> = request_line_str.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(format!(
            "Invalid request string {}",
            request_line_str
        ).into());
    }
    Ok(request_line_str)
}

fn parse_headers(
    lines: &mut Lines<&mut BufReader<TcpStream>>,
) -> Result<HashMap<String, String>> {
    let mut headers: HashMap<String, String> = HashMap::new();
    loop {
        let line = lines
            .next()
            .ok_or_else(||"Failed to read line")??;
        if line.is_empty() {
            break;
        }
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err("Invalid header format".into());
        }

        let name = parts[0].trim().to_string().to_lowercase();
        let value = parts[1].trim().to_string().to_lowercase();
        headers.insert(name, value);
    }
    Ok(headers)
}
