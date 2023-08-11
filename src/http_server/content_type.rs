use std::str::FromStr;

use crate::utils::AnyErr;

#[derive(PartialEq)]
pub enum ContentType {
    Html,
    PlainTest,
    Json,
    Ico,
    FormUrlEncoded,
    Css,
    Jpeg,
}

impl ContentType {
    fn as_str(&self) -> &'static str {
        match *self {
            ContentType::Html => "text/html",
            ContentType::Json => "application/json",
            ContentType::PlainTest => "text/plain",
            ContentType::Ico => "image/x-icon",
            ContentType::FormUrlEncoded => "application/x-www-form-urlencoded",
            ContentType::Css => "text/css",
            ContentType::Jpeg => "image/jpeg",
        }
    }
}
impl FromStr for ContentType {
    type Err = AnyErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text/html" => Ok(ContentType::Html),
            "application/json" => Ok(ContentType::Json),
            "text/plain" => Ok(ContentType::PlainTest),
            "image/x-icon" => Ok(ContentType::Ico),
            "application/x-www-form-urlencoded" => Ok(ContentType::FormUrlEncoded),
            "text/css" => Ok(ContentType::Css),
            "image/jpeg" => Ok(ContentType::Jpeg),
            _ => Err(AnyErr::new(format!("Invalid content type {}", s))),
        }
    }
}
impl ToString for ContentType {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
