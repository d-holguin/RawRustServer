use std::sync::Arc;

use crate::{
    database::Database,
    http_server::{
        AuthResult, AuthRouteHandler, ContentType, Request, Response, ResponseBuilder, RouteHandler,
    },
};

use crate::error::Result;

pub struct HomeHandler {
    pub database: Arc<Database>,
}
impl AuthRouteHandler for HomeHandler {
    fn database(&self) -> Arc<Database> {
        Arc::clone(&self.database)
    }
}

impl RouteHandler for HomeHandler {
    fn handle(&self, request: Request) -> Result<Response> {
        let body = include_str!("./home.html").to_string();

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
