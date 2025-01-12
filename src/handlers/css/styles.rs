use crate::{
    http_server::{ContentType, Request, Response, ResponseBuilder, RouteHandler}
};

use crate::error::Result;

pub struct CssHandler;
impl RouteHandler for CssHandler {
    fn handle(&self, _request: Request) -> Result<Response> {
        let css = include_str!("./styles.css").to_string();

        Ok(ResponseBuilder::new()
            .content_type(ContentType::Css)
            .body_string(css)
            .build())
    }
}
