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
        .add_route(HttpMethod::get("/home"), home)
        .add_route(HttpMethod::get("/favicon.ico"), favicon)
        .add_route(HttpMethod::post("/login"), post_login)
        .add_route(HttpMethod::get("/login"), get_login);

    let server = ServerBuilder::new()
        .address("127.0.0.1:8000")
        .thread_count(4)
        .router(router)
        .build()?;
    server.run()
}
fn get_login(_request: Request) -> Result<Response, AnyErr> {
    let html = include_str!("../assets/login.html");

    Ok(ResponseBuilder::new()
        .content_type(ContentType::Html)
        .body_string(html.to_string())
        .build())
}
fn post_login(request: Request) -> Result<Response, AnyErr> {
    let err_login = include_str!("../assets/error-login.html").to_string();
    if let Some(form_data) = request.form_data {
        let username = match form_data.get("username") {
            Some(username) => username,
            None => {
                return Ok(ResponseBuilder::new()
                    .content_type(ContentType::Html)
                    .body_string(err_login)
                    .build());
            }
        };

        let password = match form_data.get("password") {
            Some(password) => password,
            None => {
                return Ok(ResponseBuilder::new()
                    .content_type(ContentType::Html)
                    .body_string(err_login)
                    .build());
            }
        };

        if username != "admin" || password != "hunter12" {
            return Ok(ResponseBuilder::new()
                .content_type(ContentType::Html)
                .body_string(err_login)
                .build());
        }

        return Ok(ResponseBuilder::new().temp_redirect("/home").build());
    }

    Ok(ResponseBuilder::new()
        .content_type(ContentType::Html)
        .body_string(err_login)
        .build())
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
