use std::{sync::Arc, time::Instant};

use crate::{
    database::Database,
    http_server::{ContentType, Cookie, Request, Response, ResponseBuilder, RouteHandler},
    models::Session,
    utils::logger,
    error::Result
};

pub struct PostLoginHandler {
    pub database: Arc<Database>,
}
impl RouteHandler for PostLoginHandler {
    fn handle(&self, request: Request) -> Result<Response> {
        if let Some(form_data) = request.form_urlencoded() {
            let error_response = Ok(ResponseBuilder::new()
                .status_code(401)
                .reason_phrase("invalid login info".to_string())
                .build());
            let username = get_form_value(&form_data, "username");
            let password = get_form_value(&form_data, "password");

            if username.is_none() || password.is_none() {
                return error_response;
            }

            return match self.database.users.get(username.unwrap().to_string())? {
                Some(user) if &user.password == password.unwrap() => {
                    // Login successful
                    logger::info(format!("User: {} successful login", user.username).as_str());
                    let session_id = Session::generate_session_id();
                    let session = Session {
                        username: user.username.clone(),
                        session_id: session_id.clone(),
                        last_active: Instant::now(),
                    };
                    self.database.sessions.insert(session_id.clone(), session)?;
                    Ok(ResponseBuilder::new()
                        .cookie(Cookie::new("session_id", session_id))
                        .build())
                }
                _ => {
                    logger::error("Invalid Login Credentials");
                    error_response
                }
            }
        }

        Ok(ResponseBuilder::new()
            .content_type(ContentType::Html)
            .temp_redirect("/login")
            .build())
    }
}

fn get_form_value<'a>(
    form_data: &'a std::collections::HashMap<String, String>,
    key: &str,
) -> Option<&'a String> {
    form_data.get(key)
}
