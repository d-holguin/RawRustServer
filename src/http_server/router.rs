use std::collections::HashMap;

use crate::error::Result;

use super::{HttpMethod, Request, Response, ResponseBuilder};

pub trait RouteHandler: Send + Sync {
    fn handle(&self, request: Request) -> Result<Response>;
}

pub struct Router {
    pub routes: HashMap<Route, Box<dyn RouteHandler>>,
    pub not_found_response: Response,
}

#[derive(PartialEq, Eq, Hash)]
pub struct Route {
    pub http_method: HttpMethod,
    pub path_segments: Vec<String>,
}

impl Route {
    pub fn new() -> Self {
        Route {
            http_method: HttpMethod::GET,
            path_segments: Vec::new(),
        }
    }

    pub fn http_method(mut self, method: HttpMethod) -> Self {
        self.http_method = method;
        self
    }

    pub fn path(mut self, path: String) -> Self {
        let segments: Vec<String> = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
        self.path_segments = segments;
        self
    }
    pub fn matches(&self, http_method: &HttpMethod, path: &str) -> bool {
        if &self.http_method != http_method {
            return false;
        }

        let request_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        if self.path_segments.len() != request_segments.len() {
            return false;
        }

        for (route_segment, request_segment) in
            self.path_segments.iter().zip(request_segments.iter())
        {
            if route_segment != "*" && route_segment != request_segment {
                return false;
            }
        }

        true
    }
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
            not_found_response: default_not_found_response(),
        }
    }
    pub fn route(&self, method: HttpMethod, path: &str) -> Option<&Box<dyn RouteHandler>> {
        for (route, handler) in &self.routes {
            if route.matches(&method, path) {
                return Some(handler);
            }
        }
        None
    }
    pub fn add_route<H: RouteHandler + 'static>(mut self, route: Route, handler: H) -> Self {
        self.routes.insert(route, Box::new(handler));
        self
    }
    pub fn not_found_response(mut self, response: Response) -> Self {
        self.not_found_response = response;
        self
    }
}

pub fn default_not_found_response() -> Response {
    let page_404 = include_str!("../../assets/404.html").to_string();

    ResponseBuilder::new()
        .status_code(404)
        .reason_phrase("Not Found".to_string())
        .body_string(page_404)
        .build()
}
