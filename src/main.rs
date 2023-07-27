use std::{collections::HashMap, fs::File, io::Read};

use web_server::server::{MyResult, Request, Response, ResponseBuilder, Server};

fn main() {
    match Server::new("127.0.0.1:8000", 4) {
        Ok(mut server) => {
            server.add_route("/home".to_string(), home);
            server.add_route("/favicon.ico".to_string(), favicon);
            if let Err(e) = server.run() {
                eprint!("Server error: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to start server: {}", e),
    }
}

fn home(_request: Request) -> MyResult<Response> {
    let mut headers = HashMap::new();

    let body = std::fs::read_to_string("assets/home.html")?;
    headers.insert("Content-Type".to_string(), "text/html".to_string());
    headers.insert("Content-Length".to_string(), body.len().to_string());
    Ok(ResponseBuilder::new()
        .headers(headers)
        .body(body.into_bytes())
        .build())
}

fn favicon(_request: Request) -> MyResult<Response> {
    let body = std::fs::read("assets/favicon.ico")?;

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "image/x-icon".to_string());
    headers.insert("Content-Length".to_string(), body.len().to_string());

    Ok(ResponseBuilder::new().headers(headers).body(body).build())
}
