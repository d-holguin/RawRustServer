use std::collections::HashMap;

use crate::utils::AnyErr;

use super::{HttpMethod, Request, Response, ResponseBuilder};

type RouteHandler = Box<dyn Fn(Request) -> Result<Response, AnyErr> + Send + Sync + 'static>;
pub struct Router {
    pub routes: HashMap<Route, RouteHandler>,
    pub not_found_response: Response,
}

#[derive(PartialEq, Eq, Hash)]
pub struct Route {
    pub http_method: HttpMethod,
    pub path: String,
}

impl Route {
    pub fn new() -> Self {
        Route {
            http_method: HttpMethod::GET,
            path: "/".to_string(),
        }
    }

    pub fn http_method(mut self, method: HttpMethod) -> Self {
        self.http_method = method;
        self
    }

    pub fn path(mut self, path: String) -> Self {
        self.path = path;
        self
    }
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
            not_found_response: default_not_found_response().unwrap(),
        }
    }
    pub fn add_route<F>(mut self, route: Route, handler: F) -> Self
    where
        F: Fn(Request) -> Result<Response, AnyErr> + Send + Sync + 'static,
    {
        self.routes.insert(route, Box::new(handler));
        self
    }
    pub fn not_found_response(mut self, response: Response) -> Self {
        self.not_found_response = response;
        self
    }
}

fn default_not_found_response() -> Result<Response, AnyErr> {
    let page_404 = include_str!("../../assets/404.html").to_string();

    Ok(ResponseBuilder::new()
        .status_code(404)
        .reason_phrase("Not Found".to_string())
        .body_string(page_404)
        .build())
}
