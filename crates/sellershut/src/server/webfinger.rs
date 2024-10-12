use std::str::FromStr;

use activitypub_federation::{
    config::Data,
    fetch::webfinger::{build_webfinger_response, extract_webfinger_name, Webfinger},
};
use axum::{debug_handler, extract::Query, Json};
use infra::services::cache::{PoolLike, PooledConnectionLike};
use serde::Deserialize;
use tracing::{info_span, Instrument};
use url::Url;

use crate::{
    entities::{
        user::{DbUser, LocalUser},
        write_to_cache,
    },
    state::{cache::CacheKey, AppState},
};

use super::AppError;

#[derive(Deserialize)]
pub struct WebfingerQuery {
    resource: String,
}

#[debug_handler]
pub async fn webfinger(
    Query(query): Query<WebfingerQuery>,
    data: Data<AppState>,
) -> Result<Json<Webfinger>, AppError> {
    let name = extract_webfinger_name(&query.resource, &data)?;

    let cache_key = CacheKey::UserByName(name);
    let mut cache = data.services.cache.get().await?;

    let results = cache
        .get::<_, Vec<u8>>(cache_key)
        .instrument(info_span!("cache.get.user.by.name"))
        .await
        .and_then(|payload: Vec<u8>| {
            Ok(bincode::deserialize::<DbUser>(&payload).map(LocalUser::try_from))
        });
    let results: Result<Option<String>, anyhow::Error> = match results {
        Ok(Ok(Ok(data))) => Ok(Some(data.ap_id)),
        _ => {
            let db = &data.services.postgres;
            let result = sqlx::query_as!(
                DbUser,
                r#"select * from federated_user where username = $1"#,
                name
            )
            .fetch_optional(db)
            .instrument(info_span!("db.get.user"))
            .await?;

            match result {
                Some(data) => {
                    write_to_cache::<()>(cache_key, &data, cache).await?;

                    let payload = LocalUser::try_from(data)?;

                    Ok(Some(payload.ap_id))
                }
                None => Ok(None),
            }
        }
    };

    match results {
        Ok(Some(results)) => {
            let url = Url::from_str(&results)?;
            Ok(Json(build_webfinger_response(query.resource, url)))
        }
        _ => Err(AppError(anyhow::Error::msg("no user found"))),
    }
}
