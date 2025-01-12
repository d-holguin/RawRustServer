use std::sync::Arc;
use crate::database::Database;
use crate::http_server::{Request, RouteHandler};

use crate::error::Result;

pub enum AuthResult {
    Authenticated,
    SessionNotPresent,
    SessionInvalid,
}

pub trait AuthRouteHandler: RouteHandler {
    fn database(&self) -> Arc<Database>;

    fn authenticate_session(&self, request: Request) -> Result<AuthResult> {
        let cookies = request.cookies();
        let session_id = cookies.iter().find(|c| c.name == "session_id").cloned();

        match session_id {
            Some(sid) => match self.database().sessions.get(sid.value)? {
                Some(_) => Ok(AuthResult::Authenticated),
                None => Ok(AuthResult::SessionInvalid),
            },
            None => Ok(AuthResult::SessionNotPresent),
        }
    }
}
