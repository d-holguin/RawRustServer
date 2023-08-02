use std::sync::Arc;
use std::time::Instant;
use web_server::database::Database;
use web_server::http_server::{request, AuthResult, AuthRouteHandler, Cookie, RouteHandler};
use web_server::models::Session;
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
    database: Arc<Database>,
}
impl RouteHandler for PostLoginHandler {
    fn handle(&self, request: Request) -> Result<Response, AnyErr> {
        let err_login = include_str!("../assets/error-login.html").to_string();
        if let Some(form_data) = request.form_urlencoded() {
            let error_response = Ok(ResponseBuilder::new()
                .content_type(ContentType::Html)
                .body_string(err_login)
                .build());
            let username = get_form_value(&form_data, "username");
            let password = get_form_value(&form_data, "password");

            if username.is_none() || password.is_none() {
                return error_response;
            }

            match self.database.users.get(username.unwrap().to_string())? {
                Some(user) if &user.password == password.unwrap() => {
                    // Login successful
                    println!("User Login: {:?}", user);
                    let session_id = Session::generate_session_id();
                    let session = Session {
                        username: user.username.clone(),
                        session_id: session_id.clone(),
                        last_active: Instant::now(),
                    };
                    self.database.sessions.insert(session_id.clone(), session)?;
                    return Ok(ResponseBuilder::new()
                        .cookie(Cookie::new("session_id".to_string(), session_id))
                        .temp_redirect("/home")
                        .build());
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
fn get_form_value<'a>(
    form_data: &'a std::collections::HashMap<String, String>,
    key: &str,
) -> Option<&'a String> {
    form_data.get(key)
}
struct HomeHandler {
    database: Arc<Database>,
}
impl AuthRouteHandler for HomeHandler {
    fn database(&self) -> Arc<Database> {
        Arc::clone(&self.database)
    }
}
