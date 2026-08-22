pub mod error;

use sellershut_core::{RedactedSecret, category::CategoryScheme};
use sellershut_svc::cache::Cache;
use sellershut_utilities::{auth::hash_token, cache_key::CacheKey};
use sqlx::PgConnection;
use std::time::Duration;
use time::OffsetDateTime;
use tracing::{debug, info, trace};
use url::Url;
use uuid::Uuid;

use crate::error::CategoryError;

#[async_trait::async_trait]
pub trait CategoryDriver: Send + Sync {
    async fn get_category_scheme(&self, id: &Url) -> Result<Option<CategoryScheme>, CategoryError>;
    async fn upsert_scheme(
        &self,
        data: &UpsertCategoryScheme,
    ) -> Result<CategoryScheme, CategoryError>;
}

pub struct CategoryService {
    database: sqlx::PgPool,
    cache: sellershut_svc::cache::Cache,
}

pub struct UpsertCategoryScheme<'a> {
    pub ap_id: &'a Url,
    pub name: &'a str,
    pub owner_ap_id: Option<&'a str>,
    pub top_concepts_ap_id: Option<&'a str>,
    pub is_local: bool,
    pub ap_published_at: Option<&'a OffsetDateTime>,
    pub ap_updated_at: Option<&'a OffsetDateTime>,
}

#[derive(Debug)]
pub(crate) struct CategorySchemeRow {
    id: Uuid,
    ap_id: String,
    name: String,
    owner_ap_id: Option<String>,
    top_concepts_ap_id: Option<String>,
    is_local: bool,
    ap_published_at: Option<OffsetDateTime>,
    ap_updated_at: Option<OffsetDateTime>,
    last_refreshed_at: Option<OffsetDateTime>,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl TryFrom<CategorySchemeRow> for CategoryScheme {
    type Error = CategoryError;

    fn try_from(row: CategorySchemeRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,

            ap_id: Url::parse(&row.ap_id)?.into(),
            name: row.name,
            owner_ap_id: row
                .owner_ap_id
                .map(|value| Url::parse(&value).map(Into::into))
                .transpose()?,

            top_concepts_ap_id: row
                .top_concepts_ap_id
                .map(|value| Url::parse(&value).map(Into::into))
                .transpose()?,

            is_local: row.is_local,

            ap_published_at: row.ap_published_at,
            ap_updated_at: row.ap_updated_at,
            last_refreshed_at: row.last_refreshed_at,

            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

#[async_trait::async_trait]
impl CategoryDriver for CategoryService {
    async fn get_category_scheme(&self, id: &Url) -> Result<Option<CategoryScheme>, CategoryError> {
        let scheme = sqlx::query_as!(
            CategorySchemeRow,
            r#"
                 select
                 *
                 from category_scheme
                 where ap_id = $1
                 "#,
            id.as_str()
        )
        .fetch_optional(&self.database)
        .await?;
        match scheme {
            Some(scheme) => Ok(Some(CategoryScheme::try_from(scheme)?)),
            None => Ok(None),
        }
    }

    async fn upsert_scheme(
        &self,
        data: &UpsertCategoryScheme,
    ) -> Result<CategoryScheme, CategoryError> {
        /*
         * For an existing remote row, this always performs an UPDATE so
         * last_refreshed_at is recorded.
         *
         * The content fields are replaced only when the incoming object's
         * `updated` timestamp is not older than the cached version.
         *
         * A local row is not updated because of:
         *
         *     where existing.is_local = false
         *
         * If that condition rejects the update, the second SELECT returns
         * the existing row so Rust can detect and reject the local conflict.
         */
        let scheme = sqlx::query_as!(
            CategoryScheme,
            r#"
            with upserted as (
                insert into category_scheme as existing (
                    id,
                    ap_id,
                    name,
                    owner_ap_id,
                    top_concepts_ap_id,
                    is_local,
                    ap_published_at,
                    ap_updated_at,
                    last_refreshed_at
                )
                values (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5,
                    $6,
                    $7,
                    $8,
                    now()
                )
                on conflict (ap_id) do update
                set
                    name = case
                        when existing.ap_updated_at is null
                          or excluded.ap_updated_at >= existing.ap_updated_at
                        then excluded.name
                        else existing.name
                    end,

                    owner_ap_id = case
                        when existing.ap_updated_at is null
                          or excluded.ap_updated_at >= existing.ap_updated_at
                        then excluded.owner_ap_id
                        else existing.owner_ap_id
                    end,

                    top_concepts_ap_id = case
                        when existing.ap_updated_at is null
                          or excluded.ap_updated_at >= existing.ap_updated_at
                        then excluded.top_concepts_ap_id
                        else existing.top_concepts_ap_id
                    end,

                    ap_published_at = case
                        when existing.ap_updated_at is null
                          or excluded.ap_updated_at >= existing.ap_updated_at
                        then excluded.ap_published_at
                        else existing.ap_published_at
                    end,

                    ap_updated_at = case
                        when existing.ap_updated_at is null
                          or excluded.ap_updated_at >= existing.ap_updated_at
                        then excluded.ap_updated_at
                        else existing.ap_updated_at
                    end,

                    last_refreshed_at = now()

                where existing.is_local = false

                returning *
            ),

            candidate as (
                select *
                from upserted

                union all

                select *
                from category_scheme
                where ap_id = $2
                  and not exists (
                      select 1
                      from upserted
                  )
            )

            select
                id as "id!",
                ap_id as "ap_id!: _",
                name as "name!",

                owner_ap_id as "owner_ap_id?: _",
                top_concepts_ap_id as "top_concepts_ap_id?: _",

                is_local as "is_local!",

                ap_published_at as "ap_published_at?",
                ap_updated_at as "ap_updated_at?",
                last_refreshed_at as "last_refreshed_at?",

                created_at as "created_at!",
                updated_at as "updated_at!"
            from candidate
            limit 1
            "#,
            Uuid::now_v7(),
            data.ap_id.as_str(),
            data.name,
            data.owner_ap_id,
            data.top_concepts_ap_id,
            data.is_local,
            data.ap_published_at,
            data.ap_updated_at,
        )
        .fetch_one(&self.database)
        .await?;

        Ok(scheme)
    }
}

impl CategoryService {
    pub fn new(pool: sqlx::PgPool, cache: Cache) -> Self {
        Self {
            database: pool,
            cache,
        }
    }
}
