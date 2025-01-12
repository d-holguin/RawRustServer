use std::fmt::Display;
use std::str::FromStr;

#[derive(PartialEq)]
pub enum ContentType {
    Html,
    PlainTest,
    Json,
    Ico,
    FormUrlEncoded,
    Css,
    Jpeg,
    Png,
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
            ContentType::Png => "image/png",
        }
    }
}
impl FromStr for ContentType {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text/html" => Ok(ContentType::Html),
            "application/json" => Ok(ContentType::Json),
            "text/plain" => Ok(ContentType::PlainTest),
            "image/x-icon" => Ok(ContentType::Ico),
            "application/x-www-form-urlencoded" => Ok(ContentType::FormUrlEncoded),
            "text/css" => Ok(ContentType::Css),
            "image/jpeg" => Ok(ContentType::Jpeg),
            "image/png" => Ok(ContentType::Png),
            _ => Err(format!("Invalid content type {}", s)),
        }
    }
}
impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str().to_string())
    }
}
