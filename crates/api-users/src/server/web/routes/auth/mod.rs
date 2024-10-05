use anyhow::Context;
use async_session::{Session, SessionStore};
use axum::{
    http::{header::SET_COOKIE, HeaderMap},
    response::{IntoResponse, Redirect},
};
use oauth2::{basic::BasicClient, reqwest::async_http_client, AuthorizationCode, TokenResponse};
use serde::Deserialize;
use session::PostgresSessionStore;

use crate::server::AppError;

pub mod github;
pub mod session;

static COOKIE_NAME: &str = "SESSION";

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum OAuthProvider {
    GitHub,
    Discord,
}

impl OAuthProvider {
    pub fn token_url(&self) -> String {
        match self {
            OAuthProvider::GitHub => "https://github.com/login/oauth/access_token",
            OAuthProvider::Discord => todo!(),
        }
        .to_string()
    }

    async fn get_user(&self, token: &str) -> String {
        String::default()
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    code: String,
    state: String,
}

pub async fn login_authorised(
    client: BasicClient,
    provider: OAuthProvider,
    query: AuthRequest,
    store: PostgresSessionStore,
) -> Result<impl IntoResponse, AppError> {
    let token = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .context("failed in sending request request to authorization server")?;

    let user = provider.get_user(token.access_token().secret()).await;

    let mut session = Session::new();
    session.insert("user", user).unwrap();

    // Store session and get corresponding cookie
    let cookie = store
        .store_session(session)
        .await
        .context("failed to store session")?
        .context("unexpected error retrieving cookie value")?;

    // Build the cookie
    let cookie = format!("{COOKIE_NAME}={cookie}; SameSite=Lax; Path=/");

    // Set cookie
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );

    Ok((headers, Redirect::to("/")))
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
