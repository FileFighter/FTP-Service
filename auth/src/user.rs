use libunftp::auth::UserDetail;
use std::fmt::{Debug, Display};

pub struct FileFighterUser {
    username: String,
    token: String,
}

impl FileFighterUser {
    pub fn new(username: String, token: String) -> Self {
        Self { username, token }
    }
}

impl Display for FileFighterUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.username)
    }
}

impl Debug for FileFighterUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileFighterUser")
            .field("username", &self.username)
            .field("token", &"**hidden**".to_owned())
            .finish()
    }
}

impl UserDetail for FileFighterUser {
    fn account_enabled(&self) -> bool {
        true
    }
}
