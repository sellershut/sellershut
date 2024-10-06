use anyhow::Context;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::{headers, TypedHeader};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};
use secrecy::ExposeSecret;

use crate::{entity::auth::OauthDetails, server::AppError, state::AppState};

use super::{login_authorised, AuthRequest, OAuthProvider};

pub async fn github_auth(State(state): State<AppState>) -> impl IntoResponse {
    let client = state.github_client;
    // TODO: this example currently doesn't validate the CSRF token during login attempts. That
    // makes it vulnerable to cross-site request forgery. If you copy code from this example make
    // sure to add a check for the CSRF token.
    //
    // Issue for adding check to this example https://github.com/tokio-rs/axum/issues/2511
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();

    // Redirect to GitHub's oauth service
    Redirect::to(auth_url.as_ref())
}

pub fn github_oauth_client(
    oauth: &OauthDetails,
    provider: OAuthProvider,
) -> Result<BasicClient, AppError> {
    let auth_url =
        AuthUrl::new(oauth.auth_url.to_string()).expect("Invalid authorization endpoint URL");

    let token_url = TokenUrl::new(provider.token_url()).expect("Invalid token endpoint URL");

    Ok(BasicClient::new(
        ClientId::new(oauth.client_id.to_string()),
        Some(ClientSecret::new(
            oauth.client_secret.expose_secret().to_string(),
        )),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(
        RedirectUrl::new(oauth.redirect_url.to_string())
            .context("failed to create new redirection URL")?,
    ))
}

pub async fn login_authorised_github(
    Query(query): Query<AuthRequest>,
    State(state): State<AppState>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> impl IntoResponse {
    login_authorised(
        state.github_client,
        OAuthProvider::GitHub,
        query,
        state.session_store.clone(),
        &cookies,
    )
    .await
}
