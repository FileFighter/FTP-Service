use libunftp::auth::UserDetail;
use std::fmt::Display;

#[derive(Debug)]
pub struct FileFighterUser {
    username: String,
    password: String,
}

impl FileFighterUser {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

impl Display for FileFighterUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.username)
    }
}

impl UserDetail for FileFighterUser {
    fn account_enabled(&self) -> bool {
        true
    }
}
