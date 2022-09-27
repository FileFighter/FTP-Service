use libunftp::auth::UserDetail;
use std::fmt::{Debug, Display};

pub struct FileFighterUser {
    pub id: u32,
    pub username: String,
    pub token: String,
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
            .finish()
    }
}

impl UserDetail for FileFighterUser {
    fn account_enabled(&self) -> bool {
        true
    }
}
