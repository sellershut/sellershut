pub mod error;

use std::collections::HashMap;
use time::{Duration, OffsetDateTime};

use async_trait::async_trait;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet, PkceCodeChallenge,
    RedirectUrl, Scope, TokenUrl,
};
use sellershut_core::{auth::OauthProvider, types::RedactedSecret};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use url::Url;

use crate::error::AuthError;

type BasicClient = oauth2::basic::BasicClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Configuration {
    client_id: String,
    client_secret: RedactedSecret,
    redirect_url: Url,
    auth_url: Url,
    token_url: Url,
}

impl Default for Configuration {
    fn default() -> Self {
        let url = Url::parse("http://example.url").expect("valid url");
        Self {
            client_id: Default::default(),
            client_secret: Default::default(),
            redirect_url: url.clone(),
            auth_url: url.clone(),
            token_url: url,
        }
    }
}

pub const OAUTH_STATE_MAX_AGE_SECONDS: i64 = 10 * 60;
pub const ONBOARDING_MAX_AGE_SECONDS: i64 = 15 * 60;
pub const SESSION_MAX_AGE_SECONDS: i64 = 30 * 24 * 60 * 60;

impl From<Configuration> for BasicClient {
    fn from(value: Configuration) -> Self {
        oauth2::basic::BasicClient::new(ClientId::new(value.client_id))
            .set_client_secret(ClientSecret::new(value.client_secret.expose()))
            .set_auth_uri(AuthUrl::from_url(value.auth_url))
            .set_token_uri(TokenUrl::from_url(value.token_url))
            .set_redirect_uri(RedirectUrl::from_url(value.redirect_url))
    }
}

pub struct AuthorizationStart {
    pub authorisation_url: String,
    pub browser_state: String,
}

#[async_trait::async_trait]
pub trait OauthDriver: Send + Sync {
    fn providers(&self) -> Vec<OauthProvider>;
    async fn start_oauth(&self, provider: OauthProvider) -> Result<AuthorizationStart, AuthError>;
}

pub struct AuthService {
    database: sqlx::PgPool,
    providers: HashMap<OauthProvider, BasicClient>,
    http_client: reqwest::Client,
    oauth_flow_ttl: Duration,
    onboarding_ttl: Duration,
    session_ttl: Duration,
}

impl AuthService {
    pub fn new(
        pool: sqlx::PgPool,
        config: HashMap<OauthProvider, Configuration>,
    ) -> Result<Self, AuthError> {
        let mut providers = HashMap::with_capacity(config.len());

        for (k, v) in config.into_iter() {
            let client = v.into();
            match k {
                OauthProvider::Discord => {
                    providers.insert(OauthProvider::Discord, client);
                }
                OauthProvider::Google => {
                    providers.insert(OauthProvider::Google, client);
                }
                _ => todo!(),
            }
        }

        if providers.is_empty() {
            return Err(AuthError::Configuration(
                "at least one OAuth provider must be configured".to_owned(),
            ));
        }

        let http = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .user_agent("sellershut")
            .build()?;

        Ok(Self {
            database: pool,
            providers,
            http_client: http,
            oauth_flow_ttl: Duration::seconds(OAUTH_STATE_MAX_AGE_SECONDS),
            onboarding_ttl: Duration::seconds(ONBOARDING_MAX_AGE_SECONDS),
            session_ttl: Duration::seconds(SESSION_MAX_AGE_SECONDS),
        })
    }

    fn configured_provider(&self, provider: OauthProvider) -> Result<&BasicClient, AuthError> {
        self.providers
            .get(&provider)
            .ok_or_else(|| AuthError::UnsupportedProvider(provider.to_string()))
    }
}

#[async_trait]
impl OauthDriver for AuthService {
    fn providers(&self) -> Vec<OauthProvider> {
        [OauthProvider::Google, OauthProvider::Discord]
            .into_iter()
            .filter(|provider| self.providers.contains_key(provider))
            .collect()
    }

    async fn start_oauth(&self, provider: OauthProvider) -> Result<AuthorizationStart, AuthError> {
        let configured = self.configured_provider(provider)?;

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let scopes = provider.scopes();
        let scopes = scopes.iter().map(|scope| Scope::new((*scope).to_owned()));

        let (authorization_url, state) = configured
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes)
            .set_pkce_challenge(pkce_challenge)
            .url();

        let browser_state = state.secret().to_owned();
        let state_hash = hash_token(&browser_state);
        let expires_at = expires_at(self.oauth_flow_ttl)?;

        sqlx::query!(
            "insert into oauth_flow (state_hash, provider, pkce_verifier, expires_at)
            values ($1, $2, $3, $4)
        ",
            state_hash,
            provider.to_string(),
            pkce_verifier.secret(),
            expires_at
        )
        .execute(&self.database)
        .await?;

        Ok(AuthorizationStart {
            authorisation_url: authorization_url.to_string(),
            browser_state,
        })
    }
    //
    // async fn finish_oauth(
    //     &self,
    //     provider: Provider,
    //     code: &str,
    //     callback_state: &str,
    //     browser_state: &str,
    // ) -> Result<LoginOutcome, AuthError> {
    //     if hash_token(callback_state) != hash_token(browser_state) {
    //         return Err(AuthError::InvalidOAuthState);
    //     }
    //
    //     let flow = sqlx::query_as::<_, OAuthFlowRow>(
    //         r#"
    //         DELETE FROM oauth_flows
    //         WHERE state_hash = $1
    //           AND provider = $2
    //           AND expires_at > now()
    //         RETURNING pkce_verifier
    //         "#,
    //     )
    //     .bind(hash_token(callback_state))
    //     .bind(provider.as_str())
    //     .fetch_optional(&self.pool)
    //     .await?
    //     .ok_or(AuthError::InvalidOAuthState)?;
    //
    //     let configured = self.configured_provider(provider)?;
    //     let token = configured
    //         .client
    //         .exchange_code(AuthorizationCode::new(code.to_owned()))
    //         .set_pkce_verifier(PkceCodeVerifier::new(flow.pkce_verifier))
    //         .request_async(&self.http)
    //         .await
    //         .map_err(|error| AuthError::TokenExchange(error.to_string()))?;
    //
    //     let profile = configured
    //         .fetch_profile(&self.http, token.access_token().secret())
    //         .await?;
    //
    //     self.resolve_profile(profile).await
    // }
    //
    // async fn complete_onboarding(
    //     &self,
    //     onboarding_token: &str,
    //     username: &str,
    // ) -> Result<AuthenticatedSession, AuthError> {
    //     let (username, username_normalized) = validate_username(username)?;
    //     let mut tx = self.pool.begin().await?;
    //
    //     // DELETE ... RETURNING consumes the token atomically. Any later error rolls the transaction
    //     // back, so a username conflict does not destroy the pending login.
    //     let pending = sqlx::query_as::<_, PendingLoginRow>(
    //         r#"
    //         DELETE FROM pending_oauth_logins
    //         WHERE token_hash = $1
    //           AND expires_at > now()
    //         RETURNING provider, provider_subject, email, email_normalized
    //         "#,
    //     )
    //     .bind(hash_token(onboarding_token))
    //     .fetch_optional(tx.as_mut())
    //     .await?
    //     .ok_or(AuthError::InvalidOnboardingToken)?;
    //
    //     let provider = pending.provider.parse::<Provider>()?;
    //
    //     // Another request may have linked or created this account after the callback but before
    //     // onboarding. Re-check both the provider identity and normalized email inside this
    //     // transaction before inserting a new user.
    //     if let Some(user) =
    //         find_user_by_identity(tx.as_mut(), provider, &pending.provider_subject).await?
    //     {
    //         touch_identity(
    //             tx.as_mut(),
    //             provider,
    //             &pending.provider_subject,
    //             &pending.email,
    //         )
    //         .await?;
    //         let session = self.create_session(tx.as_mut(), user).await?;
    //         tx.commit().await?;
    //         return Ok(session);
    //     }
    //
    //     if let Some(user) = find_user_by_email(tx.as_mut(), &pending.email_normalized).await? {
    //         ensure_identity(
    //             tx.as_mut(),
    //             provider,
    //             &pending.provider_subject,
    //             user.id,
    //             &pending.email,
    //         )
    //         .await?;
    //         let session = self.create_session(tx.as_mut(), user).await?;
    //         tx.commit().await?;
    //         return Ok(session);
    //     }
    //
    //     let inserted = sqlx::query_as::<_, UserRow>(
    //         r#"
    //         INSERT INTO users (
    //             id,
    //             email,
    //             email_normalized,
    //             username,
    //             username_normalized
    //         )
    //         VALUES ($1, $2, $3, $4, $5)
    //         ON CONFLICT DO NOTHING
    //         RETURNING id, email, username, created_at
    //         "#,
    //     )
    //     .bind(Uuid::new_v4())
    //     .bind(&pending.email)
    //     .bind(&pending.email_normalized)
    //     .bind(&username)
    //     .bind(&username_normalized)
    //     .fetch_optional(tx.as_mut())
    //     .await?;
    //
    //     let user = match inserted {
    //         Some(user) => user,
    //         None => {
    //             // ON CONFLICT may mean a concurrent login just created this email, in which case
    //             // link to that user. If the email still does not exist, the username was taken.
    //             find_user_by_email(tx.as_mut(), &pending.email_normalized)
    //                 .await?
    //                 .ok_or(AuthError::UsernameTaken)?
    //         }
    //     };
    //
    //     ensure_identity(
    //         tx.as_mut(),
    //         provider,
    //         &pending.provider_subject,
    //         user.id,
    //         &pending.email,
    //     )
    //     .await?;
    //
    //     let session = self.create_session(tx.as_mut(), user).await?;
    //     tx.commit().await?;
    //     Ok(session)
    // }
    //
    // async fn user_from_session(&self, session_token: &str) -> Result<User, AuthError> {
    //     let user = sqlx::query_as::<_, UserRow>(
    //         r#"
    //         SELECT u.id, u.email, u.username, u.created_at
    //         FROM auth_sessions AS s
    //         JOIN users AS u ON u.id = s.user_id
    //         WHERE s.token_hash = $1
    //           AND s.expires_at > now()
    //         "#,
    //     )
    //     .bind(hash_token(session_token))
    //     .fetch_optional(&self.pool)
    //     .await?
    //     .ok_or(AuthError::InvalidSession)?;
    //
    //     Ok(user.into())
    // }
    //
    // async fn revoke_session(&self, session_token: &str) -> Result<(), AuthError> {
    //     sqlx::query("DELETE FROM auth_sessions WHERE token_hash = $1")
    //         .bind(hash_token(session_token))
    //         .execute(&self.pool)
    //         .await?;
    //
    //     Ok(())
    // }
}

fn hash_token(token: &str) -> Vec<u8> {
    Sha256::digest(token.as_bytes()).to_vec()
}

fn expires_at(ttl: Duration) -> Result<OffsetDateTime, AuthError> {
    let ttl = Duration::try_from(ttl)
        .map_err(|_| AuthError::Configuration("auth TTL is too large".to_owned()))?;

    OffsetDateTime::now_utc()
        .checked_add(ttl)
        .ok_or_else(|| AuthError::Configuration("auth TTL overflows the clock".to_owned()))
}
