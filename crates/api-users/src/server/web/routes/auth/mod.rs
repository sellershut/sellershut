use secrecy::SecretString;
use serde::Deserialize;

use crate::state::AppState;

pub mod github;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum OAuthProvider {
    GitHub,
    Discord,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Debug, Deserialize)]
struct OAuth {
    github: OAuthConfig,
    discord: OAuthConfig,
}

#[derive(Debug, Deserialize)]
struct OAuthConfig {
    client_id: String,
    client_secret: SecretString,
}


// async fn csrf_token_validation_workflow(
//     auth_request: &AuthRequest,
//     cookies: &headers::Cookie,
//     store: &AppState,
// ) -> Result<(), AppError> {
//     // Extract the cookie from the request
//     let cookie = cookies
//         .get(COOKIE_NAME)
//         .context("unexpected error getting cookie name")?
//         .to_string();
//
//     // Load the session
//     let session = match store
//         .load_session(cookie)
//         .await
//         .context("failed to load session")?
//     {
//         Some(session) => session,
//         None => return Err(anyhow!("Session not found").into()),
//     };
//
//     // Extract the CSRF token from the session
//     let stored_csrf_token = session
//         .get::<CsrfToken>("csrf_token")
//         .context("CSRF token not found in session")?
//         .to_owned();
//
//     // Cleanup the CSRF token session
//     store
//         .destroy_session(session)
//         .await
//         .context("Failed to destroy old session")?;
//
//     // Validate CSRF token is the same as the one in the auth request
//     if *stored_csrf_token.secret() != auth_request.state {
//         return Err(anyhow!("CSRF token mismatch").into());
//     }
//
//     Ok(())
// }
