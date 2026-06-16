use anyhow::Context;
use auth::discord::DiscordUser;
use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use tower_sessions::Session;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    server::{
        error::AppError,
        routes::auth::{OauthParams, SESSION_PENDING_OAUTH_ID, SESSION_USER_ID},
    },
    state::AppState,
};

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[allow(dead_code)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}

/// Authorised callback
#[utoipa::path(
    get,
    responses(
        (
            status = 200,
            description = "Application authorised",
            headers(
                ("x-request-id", description = "Request identifier"),
                ("set-cookie", description = "Oauth session cookie")
            )
        ),
    ),
    operation_id = "authorised", // https://github.com/juhaku/utoipa/issues/1170
    path = "/auth/{provider}/authorised",
    tag = super::AUTH,
    params(AuthRequest, OauthParams)
)]
pub async fn authorised(
    Query(params): Query<AuthRequest>,
    State(state): State<AppState>,
    Path(provider): Path<super::OauthProvider>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let session_state = super::session_oauth_state_key(provider);
    let expected_state = session
        .get::<String>(&session_state)
        .await?
        .context("session not in store")?;

    if params.state != expected_state {
        return Err(anyhow::anyhow!("state mismatch").into());
    }

    let _: Option<String> = session.remove(&session_state).await?;

    let client = state
        .oauth_clients
        .get(&provider)
        .context("oauth provider not configured")?;

    let http_client = state.http_client.clone();

    let user = match provider {
        crate::server::OauthProvider::Discord => auth::get_oauth_user::<DiscordUser>(
            client,
            &params.code,
            http_client,
            // https://discord.com/developers/docs/resources/user#get-current-user
            "https://discordapp.com/api/users/@me",
        ),
    }
    .await?;

    let provider_str = provider.to_string();

    let mut tx = state.database.begin().await?;
    if let Some(user_id) = sqlx::query_scalar!(
        "
        select
            user_id
        from
            account
        where
            provider_id = $1
        and
            provider_user_id = $2
        ",
        &provider_str,
        user.id
    )
    .fetch_optional(&mut *tx)
    .await?
    {
        // update the user data
        sqlx::query!(
            "
            update account
            set
                email = $1
            where
                provider_id = $2
            and provider_user_id = $3
        ",
            &user.email,
            &provider_str,
            user.id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        login_session(&session, &user_id).await?;

        // redirect to frontend's post login
        return Ok(Redirect::to("/"));
    };

    // NOTE: Account linking here: User exists but has another account
    if let Some(row) = sqlx::query_scalar!(
        "
        select 
            user_id
        from
            account
        where
            email = $1
        ",
        &user.email
    )
    .fetch_optional(&mut *tx)
    .await?
    {
        sqlx::query!(
            "
            insert into account (
                provider_id,
                provider_user_id,
                email,
                user_id
            )
            values ($1, $2, $3, $4)
            ",
            provider_str,
            user.id,
            user.email,
            &row
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        login_session(&session, &row).await?;

        // redirect to frontend's post login
        return Ok(Redirect::to("/"));
    }

    tx.commit().await?;

    let pending_id = Uuid::now_v7();
    let account_data = serde_json::to_vec(&user)?;

    let pending_key = pending_oauth_key(&pending_id);

    state
        .cache
        .set_ex(
            crate::server::cache_key::CacheKey::Session(&pending_key),
            &account_data,
            30 * 60,
        )
        .await?;

    session
        .insert(SESSION_PENDING_OAUTH_ID, pending_id.to_string())
        .await?;

    Ok(Redirect::to("http://localhost:5173/complete-profile"))
}

async fn login_session(session: &Session, user_id: &Uuid) -> Result<(), AppError> {
    // for session changing from pending to logged-in
    session.cycle_id().await?;
    session.remove::<String>(SESSION_PENDING_OAUTH_ID).await?;
    session.insert(SESSION_USER_ID, user_id.to_string()).await?;

    Ok(())
}

fn pending_oauth_key(id: &Uuid) -> String {
    format!("pending_oauth:{id}")
}
