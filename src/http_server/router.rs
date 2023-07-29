use std::collections::HashMap;

use super::{MyResult, Request, Response, ResponseBuilder};

type RouterHandler = Box<dyn Fn(Request) -> MyResult<Response> + Send + Sync>;
pub struct Router {
    pub routes: HashMap<&'static str, RouterHandler>,
    pub not_found_response: Response,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
            not_found_response: default_not_found_response().unwrap(),
        }
    }
    pub fn add_route<F>(mut self, route: &'static str, handler: F) -> Self
    where
        F: Fn(Request) -> MyResult<Response> + 'static + Send + Sync,
    {
        self.routes.insert(route, Box::new(handler));
        self
    }
    pub fn not_found_response(mut self, response: Response) -> Self {
        self.not_found_response = response;
        self
    }
}

fn default_not_found_response() -> MyResult<Response> {
    let page_404 = include_str!("../../assets/404.html").to_string();

    Ok(ResponseBuilder::new()
        .status_code(404)
        .reason_phrase("Not Found".to_string())
        .body_string(page_404)
        .build())
}
