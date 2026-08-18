pub mod error;
pub(crate) mod profile;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use oauth2_reqwest::ReqwestClient;
use rand::{TryRng, rngs::SysRng};
use utoipa::ToSchema;
use std::collections::HashMap;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use async_trait::async_trait;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use sellershut_core::{auth::OauthProvider, types::RedactedSecret};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use url::Url;

use crate::{error::AuthError, profile::OAuthProfile};

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

#[derive(Deserialize, Serialize, ToSchema)]
pub struct AuthorizationStart {
    pub authorisation_url: String,
    pub state: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub created_at: OffsetDateTime,
}

pub struct AuthenticatedSession {
    /// The raw session token. Store it in an HttpOnly cookie; do not persist it in plaintext.
    pub token: String,
    pub user: User,
}

pub enum LoginOutcome {
    Authenticated(AuthenticatedSession),
    OnboardingRequired {
        /// The raw onboarding token. In an HttpOnly cookie.
        onboarding_token: String,
    },
}

#[async_trait::async_trait]
pub trait OauthDriver: Send + Sync {
    fn providers(&self) -> Vec<OauthProvider>;
    async fn start_oauth(&self, provider: OauthProvider) -> Result<AuthorizationStart, AuthError>;
    async fn authorise(
        &self,
        provider: OauthProvider,
        code: &str,
        state: &str,
    ) -> Result<LoginOutcome, AuthError>;
}

pub struct AuthService {
    database: sqlx::PgPool,
    providers: HashMap<OauthProvider, BasicClient>,
    http_client: ReqwestClient,
    reqwest_client: reqwest::Client,
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
            http_client: ReqwestClient::from(http.clone()),
            oauth_flow_ttl: Duration::seconds(OAUTH_STATE_MAX_AGE_SECONDS),
            onboarding_ttl: Duration::seconds(ONBOARDING_MAX_AGE_SECONDS),
            session_ttl: Duration::seconds(SESSION_MAX_AGE_SECONDS),
            reqwest_client: http,
        })
    }

    fn configured_provider(&self, provider: OauthProvider) -> Result<&BasicClient, AuthError> {
        self.providers
            .get(&provider)
            .ok_or_else(|| AuthError::UnsupportedProvider(provider.to_string()))
    }

    async fn resolve_profile(&self, profile: OAuthProfile) -> Result<LoginOutcome, AuthError> {
        let OAuthProfile {
            provider,
            id,
            email,
        } = profile;
        let email = email.trim().to_lowercase();

        let mut tx = self.database.begin().await?;
        if let Some(user) = find_user_by_identity(tx.as_mut(), provider, &id).await? {
            touch_identity(tx.as_mut(), provider, &id, &email).await?;
            let session = self.create_session(tx.as_mut(), user).await?;
            tx.commit().await?;
            return Ok(LoginOutcome::Authenticated(session));
        }

        if let Some(user) = find_user_by_email(tx.as_mut(), &email).await? {
            ensure_identity(tx.as_mut(), provider, &id, user.id, &email).await?;
            let session = self.create_session(tx.as_mut(), user).await?;
            tx.commit().await?;
            return Ok(LoginOutcome::Authenticated(session));
        }

        let onboarding_token = random_token();
        let onboarding_token_hash = hash_token(&onboarding_token);
        let expires_at = expires_at(self.onboarding_ttl)?;

        sqlx::query!(
            r#"
            insert into pending_oauth_login (
                token_hash,
                provider,
                provider_subject,
                email,
                expires_at
            )
            values ($1, $2, $3, $4, $5)
            on conflict (provider, provider_subject)
            do update set
                token_hash = excluded.token_hash,
                email = excluded.email,
                expires_at = excluded.expires_at,
                created_at = now()
            "#,
            onboarding_token_hash,
            provider.to_string(),
            &id,
            &email,
            expires_at,
        )
        .execute(tx.as_mut())
        .await?;

        tx.commit().await?;

        Ok(LoginOutcome::OnboardingRequired {
            onboarding_token: onboarding_token.to_owned(),
        })
    }

    async fn create_session(
        &self,
        connection: &mut sqlx::PgConnection,
        user: User,
    ) -> Result<AuthenticatedSession, AuthError> {
        let token = random_token();
        let token_hash = hash_token(&token);
        let expires_at = expires_at(self.session_ttl)?;

        sqlx::query!(
            r#"
            insert into auth_session (token_hash, user_id, expires_at)
            values ($1, $2, $3)
            "#,
            token_hash,
            user.id,
            expires_at
        )
        .execute(connection)
        .await?;

        Ok(AuthenticatedSession { token, user })
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

        let state = state.secret().to_owned();
        let state_hash = hash_token(&state);
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
            state,
        })
    }

    async fn authorise(
        &self,
        provider: OauthProvider,
        code: &str,
        state: &str,
    ) -> Result<LoginOutcome, AuthError> {
        let pkce_verifier = sqlx::query_scalar!(
            "
             delete from oauth_flow
             where state_hash = $1
               and provider = $2
               and expires_at > now()
             returning pkce_verifier
             ",
            hash_token(state),
            provider.to_string()
        )
        .fetch_optional(&self.database)
        .await?
        .ok_or(AuthError::InvalidOAuthState)?;

        let configured = self.configured_provider(provider)?;
        let token = configured
            .exchange_code(AuthorizationCode::new(code.to_owned()))
            .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier))
            .request_async(&self.http_client)
            .await
            .map_err(|error| AuthError::TokenExchange(error.to_string()))?;

        let profile = profile::fetch_profile(
            provider,
            &self.reqwest_client,
            token.access_token().secret(),
        )
        .await?;

        self.resolve_profile(profile).await
    }
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

fn random_token() -> String {
    let mut bytes = [0_u8; 32];
    let mut rng = SysRng;
    rng.try_fill_bytes(&mut bytes).unwrap();
    URL_SAFE_NO_PAD.encode(bytes)
}

async fn touch_identity(
    connection: &mut sqlx::PgConnection,
    provider: OauthProvider,
    provider_subject: &str,
    provider_email: &str,
) -> Result<(), AuthError> {
    sqlx::query!(
        "
        update oauth_identity
        set provider_email = $3,
            last_login_at = now()
        where provider = $1
          and provider_id = $2
        ",
        provider.to_string(),
        provider_subject,
        provider_email,
    )
    .execute(connection)
    .await?;

    Ok(())
}
async fn find_user_by_email(
    connection: &mut sqlx::PgConnection,
    email: &str,
) -> Result<Option<User>, AuthError> {
    Ok(sqlx::query_as!(
        User,
        r#"
        select id, email, username, created_at
        from "user"
        where email = $1
        for update
        "#,
        email.to_lowercase()
    )
    .fetch_optional(connection)
    .await?)
}

async fn find_user_by_identity(
    connection: &mut sqlx::PgConnection,
    provider: OauthProvider,
    provider_subject: &str,
) -> Result<Option<User>, AuthError> {
    Ok(sqlx::query_as!(
        User,
        r#"
        select u.id, u.email, u.username, u.created_at
        from oauth_identity as oi
        join "user" as u on u.id = oi.user_id
        where oi.provider = $1
          and oi.provider_id = $2
        for update of oi
        "#,
        provider.to_string(),
        provider_subject
    )
    .fetch_optional(connection)
    .await?)
}

async fn ensure_identity(
    connection: &mut sqlx::PgConnection,
    provider: OauthProvider,
    provider_subject: &str,
    user_id: Uuid,
    provider_email: &str,
) -> Result<(), AuthError> {
    sqlx::query(
        r#"
        INSERT INTO oauth_identities (
            provider,
            provider_subject,
            user_id,
            provider_email
        )
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (provider, provider_subject) DO NOTHING
        "#,
    )
    .bind(provider.to_string())
    .bind(provider_subject)
    .bind(user_id)
    .bind(provider_email)
    .execute(&mut *connection)
    .await?;

    let linked_user_id = sqlx::query_scalar::<_, Uuid>(
        r#"
        SELECT user_id
        FROM oauth_identities
        WHERE provider = $1
          AND provider_subject = $2
        "#,
    )
    .bind(provider.to_string())
    .bind(provider_subject)
    .fetch_one(&mut *connection)
    .await?;

    if linked_user_id != user_id {
        return Err(AuthError::IdentityConflict);
    }

    touch_identity(connection, provider, provider_subject, provider_email).await
}
