use std::sync::Arc;
use web_server::database::Database;
use web_server::handlers::login::{GetLoginHandler, PostLoginHandler};
use web_server::handlers::CssHandler;
use web_server::http_server::{AuthResult, AuthRouteHandler, RouteHandler};

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
    let database = Arc::new(Database::database_init()?);
    let router = Router::new()
        .add_route(
            HttpMethod::get("/home"),
            HomeHandler {
                database: Arc::clone(&database),
            },
        )
        .add_route(HttpMethod::get("/styles.css"), CssHandler)
        .add_route(HttpMethod::get("/favicon.ico"), FaviconHandler)
        .add_route(
            HttpMethod::post("/login"),
            PostLoginHandler {
                database: Arc::clone(&database),
            },
        )
        .add_route(HttpMethod::get("/images/*"), GetImageHandler)
        .add_route(HttpMethod::get("/login"), GetLoginHandler);

    let server = ServerBuilder::new()
        .address("127.0.0.1:8000")
        .thread_count(4)
        .router(router)
        .build()?;
    server.run()
}

impl RouteHandler for HomeHandler {
    fn handle(&self, request: Request) -> Result<Response, AnyErr> {
        let body = include_str!("../assets/home.html").to_string();

        let login_redirect = ResponseBuilder::new().temp_redirect("/login").build();

        match self.authenticate_session(request)? {
            AuthResult::Authenticated => Ok(ResponseBuilder::new()
                .content_type(ContentType::Html)
                .body_string(body)
                .build()),
            AuthResult::SessionNotPresent => Ok(login_redirect),
            AuthResult::SessionInvalid => Ok(login_redirect),
        }
    }
}

struct FaviconHandler;
impl RouteHandler for FaviconHandler {
    fn handle(&self, _request: Request) -> Result<Response, AnyErr> {
        let body = include_bytes!("../assets/favicon.png").to_vec();
        Ok(ResponseBuilder::new()
            .content_type(ContentType::Ico)
            .body_bytes(body)
            .build())
    }
}
struct HomeHandler {
    database: Arc<Database>,
}
impl AuthRouteHandler for HomeHandler {
    fn database(&self) -> Arc<Database> {
        Arc::clone(&self.database)
    }
}

pub struct GetImageHandler;

impl RouteHandler for GetImageHandler {
    fn handle(&self, request: Request) -> Result<Response, AnyErr> {
        let path = request.path;

        println!("PATH = {}", path);
        if !path.starts_with("/images/") {
            return Err(AnyErr::new("Invalid path"));
        }

        let full_path = format!("../assets/images/{}", path);

        let image_data = std::fs::read(&full_path)
            .map_err(|e| AnyErr::new(format!("Failed to read image: {}", e)))?;

        Ok(ResponseBuilder::new()
            .content_type(ContentType::Jpeg)
            .body_bytes(image_data)
            .build())
    }
}
