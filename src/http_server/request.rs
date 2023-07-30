use crate::utils::AnyErr;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Lines},
    net::TcpStream,
    str::FromStr,
};

use super::Route;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    GET,
    POST,
}
impl FromStr for HttpMethod {
    type Err = AnyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            _ => Err(AnyErr::new(format!("Invalid Http method {}", s))),
        }
    }
}

impl HttpMethod {
    pub fn get(path: &str) -> Route {
        Route::new()
            .http_method(HttpMethod::GET)
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
    pub fn from_reader(reader: &mut BufReader<TcpStream>) -> Result<Request, AnyErr> {
        let mut lines = reader.lines();
        let request_line = read_request_line(&mut lines)?;
        let headers = parse_headers(&mut lines)?;
        // Read body
        let mut body = Vec::new();
        if let Some(len) = headers.get("Content-Length") {
            let len: usize = len
                .parse::<usize>()
                .map_err(|e| AnyErr::wrap("Error parsing content length".to_string(), e))?;
            body.reserve(len);
            for _ in 0..len {
                let b = *reader
                    .fill_buf()
                    .map_err(|e| AnyErr::wrap("Error reading request body".to_string(), e))?
                    .first()
                    .ok_or_else(|| AnyErr::new("Unexpected EOF"))?;
                body.push(b);
                reader.consume(1);
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
            .map_err(|e| AnyErr::wrap("Invalid HTTP Method".to_string(), e))?;

        Ok(Request {
            method,
            path,
            http_version,
            headers,
            body,
        })
    }
}

fn read_request_line(lines: &mut Lines<&mut BufReader<TcpStream>>) -> Result<String, AnyErr> {
    let request_line_result = lines.next().ok_or_else(|| AnyErr::new("No request line"))?;
    let request_line_str = request_line_result.map_err(|error| {
        AnyErr::wrap(
            format!("Unable to get request string: {}", error.to_string()),
            error,
        )
    })?;
    let parts: Vec<&str> = request_line_str.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(AnyErr::new(format!(
            "Invalid request string {}",
            request_line_str
        )));
    }
    Ok(request_line_str)
}

fn parse_headers(
    lines: &mut Lines<&mut BufReader<TcpStream>>,
) -> Result<HashMap<String, String>, AnyErr> {
    let mut headers: HashMap<String, String> = HashMap::new();
    loop {
        let line = lines
            .next()
            .ok_or_else(|| AnyErr::new("Failed to read line"))??;
        if line.is_empty() {
            break;
        }
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(AnyErr::new("Invalid header format"));
        }
        let name = parts[0].trim().to_string();
        let value = parts[1].trim().to_string();
        headers.insert(name, value);
    }
    Ok(headers)
}
