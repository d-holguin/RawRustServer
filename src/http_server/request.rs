use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Lines},
    net::TcpStream,
    str::FromStr,
};

use super::{MyResult, Route};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    GET,
    POST,
}
impl FromStr for HttpMethod {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            _ => Err(From::from(format!("Invalid Http method {s}"))),
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
    pub fn from_reader(reader: &mut BufReader<TcpStream>) -> MyResult<Request> {
        let mut lines = reader.lines();
        let request_line = read_request_line(&mut lines)?;
        let headers = parse_headers(&mut lines)?;
        // Read body
        let mut body = Vec::new();
        if let Some(len) = headers.get("Content-Length") {
            let len: usize = len
                .parse::<usize>()
                .map_err(|e| -> Box<dyn std::error::Error> {
                    From::from(format!("Failed to parse Content-Length: {}", e))
                })?;
            body.reserve(len);
            for _ in 0..len {
                let b =
                    *reader
                        .fill_buf()?
                        .first()
                        .ok_or_else(|| -> Box<dyn std::error::Error> {
                            From::from("Unexpected EOF")
                        })?;
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

        let method = HttpMethod::from_str(&method_string)?;
        Ok(Request {
            method,
            path,
            http_version,
            headers,
            body,
        })
    }
}

fn read_request_line(lines: &mut Lines<&mut BufReader<TcpStream>>) -> MyResult<String> {
    let request_line = lines.next().ok_or_else(|| "No request line".to_string())?;
    let request_line_str = request_line?;
    let parts: Vec<&str> = request_line_str.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(From::from("Invalid request line".to_string()));
    }
    Ok(request_line_str)
}

fn parse_headers(
    lines: &mut Lines<&mut BufReader<TcpStream>>,
) -> MyResult<HashMap<String, String>> {
    let mut headers: HashMap<String, String> = HashMap::new();
    loop {
        let line = lines
            .next()
            .ok_or_else(|| "Failed to read line".to_string())??;
        if line.is_empty() {
            break;
        }
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(From::from("Invalid header".to_string()));
        }
        let name = parts[0].trim().to_string();
        let value = parts[1].trim().to_string();
        headers.insert(name, value);
    }
    Ok(headers)
}
