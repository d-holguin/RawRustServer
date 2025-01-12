use std::ffi::OsStr;
use std::path::Path;
use std::sync::Arc;
use web_server_core::database::Database;
use web_server_core::error::Result;
use web_server_core::handlers::{CssHandler, GetLoginHandler, HomeHandler, PostLoginHandler};
use web_server_core::http_server::{ContentType, HttpMethod, Request, Response, ResponseBuilder, RouteHandler, Router, ServerBuilder};
use web_server_core::utils::{logger, logger_backend};

fn main() {
    logger_backend::init_global_logger("logs/server.log");
    if let Err(e) = run_server() {
        eprintln!("Failed to run server: {:?}", e);
        std::process::exit(1);
    }
}
fn run_server() -> Result<()> {
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

struct FaviconHandler;
impl RouteHandler for FaviconHandler {
    fn handle(&self, _request: Request) -> Result<Response> {
        let body = include_bytes!("../assets/favicon.png").to_vec();
        Ok(ResponseBuilder::new()
            .content_type(ContentType::Ico)
            .body_bytes(body)
            .build())
    }
}

pub struct GetImageHandler;
impl RouteHandler for GetImageHandler {
    fn handle(&self, request: Request) -> Result<Response> {
        let path = request.path;
        logger::info(&format!("Request for image. PATH = {}", path));
        if !path.starts_with("/images/") {
            return Err("Invalid path".into());
        }

        let relative_path = &path["/images/".len()..];
        let full_path = Path::new("assets/images/").join(relative_path);

        logger::info(&format!(
            "Trying to access file at: {:?}",
            full_path
        ));
        // Security check
        if !full_path
            .display()
            .to_string()
            .starts_with("assets/images/")
        {
            return Err("Invalid path".into());
        }

        let ext = full_path.extension().and_then(OsStr::to_str);
        let content_type = match ext {
            Some("jpg") | Some("jpeg") => ContentType::Jpeg,
            Some("png") => ContentType::Png,
            _ => return Err("Unsupported image format".into()),
        };

        let image_data = std::fs::read(&full_path)?;

        Ok(ResponseBuilder::new()
            .content_type(content_type)
            .body_bytes(image_data)
            .build())
    }
}
