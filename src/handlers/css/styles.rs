use crate::{
    http_server::{ContentType, Request, Response, ResponseBuilder, RouteHandler},
    utils::AnyErr,
};

pub struct CssHandler;
impl RouteHandler for CssHandler {
    fn handle(&self, _request: Request) -> Result<Response, AnyErr> {
        let css = include_str!("./styles.css").to_string();

        Ok(ResponseBuilder::new()
            .content_type(ContentType::Css)
            .body_string(css)
            .build())
    }
}
