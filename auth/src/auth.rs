use crate::user::FileFighterUser;
use async_trait::async_trait;
use filefighter_api::ffs_api::{
    endpoints::{get_token_for_user, get_user_info},
    models::user_resource,
    ApiConfig,
};
use libunftp::auth::{AuthenticationError, Authenticator, Credentials};
use tracing::{debug, info, instrument};

#[derive(Debug)]
pub struct FileFighterAuthenticator {
    api_config: ApiConfig,
}

impl FileFighterAuthenticator {
    pub fn new() -> Self {
        FileFighterAuthenticator {
            api_config: ApiConfig {
                fss_base_url: "http://localhost:8080/api".to_owned(),
                fhs_base_url: "http://localhost:5000/data".to_owned(),
            },
        }
    }
}

impl Default for FileFighterAuthenticator {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Authenticator<FileFighterUser> for FileFighterAuthenticator {
    #[instrument(skip(self, creds), level = "debug")]
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

        // IDEA: the lib does cache the user?
        let token = get_token_for_user(&self.api_config, username, password)
            .await
            .map_err(|err| {
                debug!("Cought Error: {}", err);
                AuthenticationError::new(err.to_string())
            })?;

        debug!("Got token {}", token);

        let user_ressource = get_user_info(&self.api_config, &token)
            .await
            .map_err(|err| {
                debug!("Cought Error: {}", err);
                AuthenticationError::BadUser
            })?;

        debug!("Got user {:?}", user_ressource);

        Ok(FileFighterUser {
            username: username.to_owned(),
            token,
            id: user_ressource.id,
        })
    }
}
