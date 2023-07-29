use std::{collections::HashMap};

use super::{MyResult, Request, Response};

pub struct Router {
    pub routes: HashMap<&'static str, Box<dyn Fn(Request) -> MyResult<Response> + Send + Sync>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }
    pub fn add_route<F>(mut self, route: &'static str, handler: F) -> Self
    where
        F: Fn(Request) -> MyResult<Response> + 'static + Send + Sync,
    {
        self.routes.insert(route, Box::new(handler));
        self
    }
}
