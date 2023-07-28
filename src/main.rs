use web_server::server::{ContentType, MyResult, Request, Response, ResponseBuilder, Server};

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
    let body = std::fs::read_to_string("assets/home.html")?;

    Ok(ResponseBuilder::new()
        .content_type(ContentType::Html)
        .body_string(body)
        .build())
}

fn favicon(_request: Request) -> MyResult<Response> {
    let body = std::fs::read("assets/favicon.ico")?;

    Ok(ResponseBuilder::new()
        .content_type(ContentType::Ico)
        .body_bytes(body)
        .build())
}
