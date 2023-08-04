use crate::{
    http_server::{ContentType, Request, Response, ResponseBuilder, RouteHandler},
    utils::AnyErr,
};

pub struct GetLoginHandler;
impl RouteHandler for GetLoginHandler {
    fn handle(&self, _request: Request) -> Result<Response, AnyErr> {
        let html = include_str!("./login.html").to_string();

        Ok(ResponseBuilder::new()
            .content_type(ContentType::Html)
            .body_string(html)
            .build())
    }
}
