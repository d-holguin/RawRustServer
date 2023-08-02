use std::sync::Arc;

use crate::{database::Database, utils::AnyErr};

use super::{Request, RouteHandler};
pub enum AuthResult {
    Authenticated,
    SessionNotPresent,
    SessionInvalid,
}

pub trait AuthRouteHandler: RouteHandler {
    fn database(&self) -> Arc<Database>;

    fn authenticate_session(&self, request: Request) -> Result<AuthResult, AnyErr> {
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
