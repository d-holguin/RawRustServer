use std::sync::Arc;
use web_server::database::SimpleDB;
use web_server::http_server::RouteHandler;
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
    let database: Arc<SimpleDB<String, String>> = Arc::new(SimpleDB::new());
    database.insert("admin".to_string(), "hunter12".to_string())?;
    let router = Router::new()
        .add_route(HttpMethod::get("/home"), HomeHandler)
        .add_route(HttpMethod::get("/favicon.ico"), FaviconHandler)
        .add_route(
            HttpMethod::post("/login"),
            PostLoginHandler {
                database: Arc::clone(&database),
            },
        )
        .add_route(HttpMethod::get("/login"), GetLoginHandler);

    let server = ServerBuilder::new()
        .address("127.0.0.1:8000")
        .thread_count(4)
        .router(router)
        .build()?;
    server.run()
}
struct PostLoginHandler {
    database: Arc<SimpleDB<String, String>>,
}
impl RouteHandler for PostLoginHandler {
    fn handle(&self, request: Request) -> Result<Response, AnyErr> {
        let err_login = include_str!("../assets/error-login.html").to_string();
        if let Some(form_data) = request.form_data {
            let error_response = Ok(ResponseBuilder::new()
                .content_type(ContentType::Html)
                .body_string(err_login)
                .build());
            let username = match form_data.get("username") {
                Some(username) => username,
                None => {
                    return error_response;
                }
            };

            let password = match form_data.get("password") {
                Some(password) => password,
                None => {
                    return error_response;
                }
            };

            match self.database.get(username.to_string())? {
                Some(stored_password) if &stored_password == password => {
                    // Login successful
                    return Ok(ResponseBuilder::new().temp_redirect("/home").build());
                }
                _ => {
                    return error_response;
                }
            }
        }

        Ok(ResponseBuilder::new()
            .content_type(ContentType::Html)
            .body_string(err_login)
            .build())
    }
}
struct GetLoginHandler;
impl RouteHandler for GetLoginHandler {
    fn handle(&self, _request: Request) -> Result<Response, AnyErr> {
        let html = include_str!("../assets/login.html");

        Ok(ResponseBuilder::new()
            .content_type(ContentType::Html)
            .body_string(html.to_string())
            .build())
    }
}

struct HomeHandler;
impl RouteHandler for HomeHandler {
    fn handle(&self, _request: Request) -> Result<Response, AnyErr> {
        let body = include_str!("../assets/home.html").to_string();

        Ok(ResponseBuilder::new()
            .content_type(ContentType::Html)
            .body_string(body)
            .build())
    }
}
struct FaviconHandler;
impl RouteHandler for FaviconHandler {
    fn handle(&self, _request: Request) -> Result<Response, AnyErr> {
        let body = include_bytes!("../assets/favicon.ico").to_vec();
        Ok(ResponseBuilder::new()
            .content_type(ContentType::Ico)
            .body_bytes(body)
            .build())
    }
}
