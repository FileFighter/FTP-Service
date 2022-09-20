use async_trait::async_trait;
use libunftp::auth::{AuthenticationError, Authenticator, Credentials, DefaultUser};
use tracing::instrument;

#[derive(Debug)]
pub struct FileFighterAuthenticator;

impl FileFighterAuthenticator {
    pub fn new() -> Self {
        FileFighterAuthenticator {}
    }
}

#[async_trait]
impl Authenticator<DefaultUser> for FileFighterAuthenticator {
    #[instrument]
    async fn authenticate(
        &self,
        username: &str,
        creds: &Credentials,
    ) -> Result<DefaultUser, AuthenticationError> {
        Ok(DefaultUser {})
    }
}
