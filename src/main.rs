use web_server::server::{
    ContentType, MyResult, Request, Response, ResponseBuilder, Router, Server,
};

fn main() {
    let router = Router::new()
        .add_route("/home", home)
        .add_route("/favicon.ico", favicon);
    match Server::new("127.0.0.1:8000", 4, router) {
        Ok(server) => {
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

fn favicon(request: Request) -> MyResult<Response> {
    let body = std::fs::read("assets/favicon.ico")?;
    println!("Received request: {:?}", request);
    Ok(ResponseBuilder::new()
        .content_type(ContentType::Ico)
        .body_bytes(body)
        .build())
}
