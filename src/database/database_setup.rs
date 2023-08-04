use std::sync::Arc;

use crate::{
    models::{Session, User},
    utils::AnyErr,
}; 

use super::SimpleDB;

pub struct Database {
    pub users: Arc<SimpleDB<String, User>>,
    pub sessions: Arc<SimpleDB<String, Session>>,
}
impl Database {
    pub fn database_init() -> Result<Database, AnyErr> {
        let users: Arc<SimpleDB<String, User>> = Arc::new(SimpleDB::new());
        let sessions: Arc<SimpleDB<String, Session>> = Arc::new(SimpleDB::new());
        let admin_user = User {
            username: "admin".to_string(),
            password: "hunter12".to_string(),
        };
        users
            .insert(admin_user.username.clone(), admin_user)
            .map_err(|e| AnyErr::wrap("error adding admin credentials to database", e))?;

        Ok(Database { users, sessions })
    }
}
