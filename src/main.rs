use web_server::{
    http_server::{
        ContentType, HttpMethod, Request, Response, ResponseBuilder, Router, ServerBuilder,
    },
    utils::AnyErr,
};

fn main() {
    match run_server() {
        Ok(_) => println!("Server shut down successfully."),
        Err(e) => {
            eprintln!("Failed to run server: {:?}", e);
            std::process::exit(1);
        }
    }
}
fn run_server() -> Result<(), AnyErr> {
    let router = Router::new()
        .add_route(HttpMethod::get("/"), home)
        .add_route(HttpMethod::get("/favicon.ico"), favicon);

    let server = ServerBuilder::new()
        .address("127.0.0.1:8000")
        .thread_count(4)
        .router(router)
        .build()?;
    server.run()
}

fn home(_request: Request) -> Result<Response, AnyErr> {
    let body = std::fs::read_to_string("assets/home.html")?;

    Ok(ResponseBuilder::new()
        .content_type(ContentType::Html)
        .body_string(body)
        .build())
}

fn favicon(_request: Request) -> Result<Response, AnyErr> {
    let body = std::fs::read("assets/favicon.ico")?;
    Ok(ResponseBuilder::new()
        .content_type(ContentType::Ico)
        .body_bytes(body)
        .build())
}
