use async_trait::async_trait;
use libunftp::auth::{AuthenticationError, Authenticator, Credentials, DefaultUser};

#[derive(Debug)]
pub struct FileFighterAuthenticator;

impl FileFighterAuthenticator {
    pub fn new() -> Self {
        FileFighterAuthenticator {}
    }
}

#[async_trait]
impl Authenticator<DefaultUser> for FileFighterAuthenticator {
    async fn authenticate(
        &self,
        username: &str,
        creds: &Credentials,
    ) -> Result<DefaultUser, AuthenticationError> {
        todo!()
    }
}
