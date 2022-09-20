use crate::user::FileFighterUser;
use async_trait::async_trait;
use libunftp::auth::{AuthenticationError, Authenticator, Credentials};
use tracing::instrument;

#[derive(Debug)]
pub struct FileFighterAuthenticator;

impl FileFighterAuthenticator {
    pub fn new() -> Self {
        FileFighterAuthenticator {}
    }
}

#[async_trait]
impl Authenticator<FileFighterUser> for FileFighterAuthenticator {
    #[instrument]
    async fn authenticate(
        &self,
        username: &str,
        creds: &Credentials,
    ) -> Result<FileFighterUser, AuthenticationError> {
        if username.is_empty() {
            return Err(AuthenticationError::BadUser);
        }

        let password = creds
            .password
            .as_ref()
            .ok_or(AuthenticationError::BadPassword)?;

        if password.is_empty() {
            return Err(AuthenticationError::BadPassword);
        }

        Ok(FileFighterUser::new(
            username.to_owned(),
            password.to_owned(),
        ))
    }
}
