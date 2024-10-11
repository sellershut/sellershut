use futures_util::TryFutureExt;
use infra::{
    events::{Entity, Event},
    services::cache::{
        key::{CacheKey, CursorParams, Index},
        PoolLike, PooledConnection, PooledConnectionLike,
    },
};
use prost::Message;
use sellershut_core::{
    categories::{
        query_categories_server::QueryCategories, CacheCategoriesConnectionRequest, Category,
        Connection, GetCategoryRequest, GetSubCategoriesRequest, Node,
    },
    common::pagination::{self, cursor::cursor_value::CursorType, Cursor, CursorBuilder, PageInfo},
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime, UtcOffset};
use tracing::{debug, debug_span, info_span, instrument, trace, Instrument, Level};

use crate::{
    api::entity::{self},
    state::{
        database::{map_err, publish_event},
        ApiState,
    },
};

static MAX_RESULTS: i32 = 250;

#[tonic::async_trait]
impl QueryCategories for ApiState {
    #[doc = " gets all categories"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn categories(
        &self,
        request: tonic::Request<pagination::Cursor>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        // get cache first
        trace!("getting cache state");
        let cache = self
            .state
            .cache
            .get()
            .instrument(debug_span!("cache.get.pool"))
            .await
            .map_err(map_err)?;

        let pagination = request.into_inner();

        // get count
        let actual_count = pagination::query_count(
            MAX_RESULTS,
            &pagination.index.ok_or_else(|| {
                tonic::Status::new(tonic::Code::Internal, "missing pagination index")
            })?,
        );
        // get 1 more
        let get_count: i64 = actual_count as i64 + 1;

        // a cursor was specified
        let (connection, cache_ok) = if let Some(ref cursor) = pagination.cursor_value {
            // get cursor
            let cursor_value = cursor.cursor_type.as_ref().ok_or_else(|| {
                tonic::Status::new(tonic::Code::Internal, "Cursor type is not set")
            })?;

            let decode_cursor = |cursor_value: &CursorType| {
                CursorBuilder::decode(cursor_value)
                    .map_err(|e| tonic::Status::internal(e.to_string()))
            };

            let connection = match cursor_value {
                CursorType::After(cursor) => {
                    // try cache first
                    let cache_key = CacheKey::Categories(CursorParams {
                        cursor: Some(cursor),
                        index: Index::First(actual_count),
                    });

                    let cache_result = read_cache(cache_key, cache).await;
                    let is_cache_ok = cache_result.is_ok();

                    let connection = if let Ok(con) = cache_result {
                        trace!("cache ok");
                        con
                    } else {
                        let cursor = decode_cursor(cursor_value)?;
                        let id = cursor.id();
                        trace!("converting to date {:?}", cursor.dt());

                        let created_at =
                            OffsetDateTime::parse(cursor.dt(), &Rfc3339).map_err(map_err)?;

                        let fut_count = sqlx::query_scalar!(
                            "
                                    select count(*) from category
                                    where 
                                        (
                                            created_at <> $1
                                            or id <= $2
                                        )
                                        and created_at < $1
                                ",
                            created_at,
                            id,
                        )
                        .fetch_one(&self.state.postgres)
                        .instrument(debug_span!("pg.select.count"))
                        .map_err(map_err);

                        let fut_categories = sqlx::query_as!(
                            entity::Category,
                            "
                                    select * from category
                                    where 
                                        (
                                            created_at = $1
                                            and id > $2
                                        )
                                        or created_at >= $1
                                    order by
                                        created_at asc,
                                        id asc
                                    limit
                                        $3
                                ",
                            created_at,
                            id,
                            get_count
                        )
                        .fetch_all(&self.state.postgres)
                        .instrument(debug_span!("pg.select.*"))
                        .map_err(map_err);

                        let (count_on_other_end, categories) =
                            tokio::try_join!(fut_count, fut_categories)?;

                        parse_categories(count_on_other_end, categories, &pagination, actual_count)?
                    };

                    (connection, is_cache_ok)
                }
                CursorType::Before(cursor) => {
                    // try cache first
                    let cache_key = CacheKey::Categories(CursorParams {
                        cursor: Some(cursor),
                        index: Index::Last(actual_count),
                    });

                    let cache_result = read_cache(cache_key, cache).await;
                    let is_cache_ok = cache_result.is_ok();

                    let connection = if let Ok(con) = cache_result {
                        trace!("cache ok");
                        con
                    } else {
                        let cursor = decode_cursor(cursor_value)?;
                        let id = cursor.id();
                        let created_at = OffsetDateTime::parse(cursor.dt(), &Rfc3339)
                            .map_err(|e| tonic::Status::internal(e.to_string()))?;

                        let fut_count = sqlx::query_scalar!(
                            "
                                    select count(*) from category
                                    where 
                                        (
                                            created_at <> $1
                                            or id > $2
                                        )
                                        and created_at >= $1
                                ",
                            created_at,
                            id,
                        )
                        .fetch_one(&self.state.postgres)
                        .instrument(debug_span!("pg.select.count"))
                        .map_err(map_err);

                        let fut_categories = sqlx::query_as!(
                            entity::Category,
                            "
                                    select * from category
                                    where 
                                        (
                                            created_at = $1
                                            and id < $2
                                        )
                                        or created_at < $1
                                    order by
                                        created_at desc,
                                        id desc
                                    limit
                                        $3
                                ",
                            created_at,
                            id,
                            get_count
                        )
                        .fetch_all(&self.state.postgres)
                        .instrument(debug_span!("pg.select.*"))
                        .map_err(map_err);

                        let (count, categories) = tokio::try_join!(fut_count, fut_categories)?;

                        parse_categories(count, categories, &pagination, actual_count)?
                    };
                    (connection, is_cache_ok)
                }
            };
            connection
        } else {
            // try cache first
            let index = match pagination.index.expect("index to be available") {
                pagination::cursor::Index::First(count) => Index::First(count),
                pagination::cursor::Index::Last(count) => Index::Last(count),
            };
            let cache_key = CacheKey::Categories(CursorParams {
                cursor: None,
                index,
            });

            let cache_result = read_cache(cache_key, cache).await;
            let is_cache_ok = cache_result.is_ok();

            let connection = if let Ok(con) = cache_result {
                trace!("cache ok");
                con
            } else {
                let categories = match index {
                    Index::First(_) => sqlx::query_as!(
                        entity::Category,
                        "select * FROM category
                            order by
                                created_at asc
                            limit $1",
                        get_count
                    )
                    .fetch_all(&self.state.postgres)
                    .instrument(debug_span!("pg.select.*"))
                    .await
                    .map_err(map_err)?,
                    Index::Last(_) => sqlx::query_as!(
                        entity::Category,
                        "select * FROM category
                            order by
                                created_at desc
                            limit $1",
                        get_count
                    )
                    .fetch_all(&self.state.postgres)
                    .instrument(debug_span!("pg.select.*"))
                    .await
                    .map_err(map_err)?,
                };

                parse_categories(
                    Some(get_count - categories.len() as i64),
                    categories,
                    &pagination,
                    actual_count,
                )?
            };
            (connection, is_cache_ok)
        };

        if !cache_ok {
            let payload = CacheCategoriesConnectionRequest {
                connection: Some(connection.clone()),
                pagination: Some(pagination),
            };

            let event = Event::UpdateBatch(Entity::Categories);

            publish_event(payload, event, &self.state.jetstream).await?;
        }

        Ok(tonic::Response::new(connection))
    }

    #[doc = " get category by id"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn category_by_id(
        &self,
        request: tonic::Request<GetCategoryRequest>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        let state = &self.state;
        let id = request.into_inner().id;

        let cache_key = CacheKey::Category(&id);

        let s = info_span!("cache call");

        // get cache first
        let mut cache = self
            .state
            .cache
            .get()
            .instrument(debug_span!("cache.get.pool"))
            .await
            .map_err(map_err)?;
        let cache_result = cache
            .get::<_, Vec<u8>>(&cache_key)
            .map_err(|e| tonic::Status::internal(e.to_string()))
            .and_then(|payload| async move {
                if !payload.is_empty() {
                    Category::decode(payload.as_ref())
                        .map_err(|e| tonic::Status::internal(e.to_string()))
                } else {
                    let msg = "no data available in cache";
                    debug!("{}", msg);
                    Err(tonic::Status::not_found(msg))
                }
            })
            .instrument(s)
            .await;

        let category = match cache_result {
            Ok(category) => {
                trace!("cache ok");
                category
            }
            Err(_e) => {
                debug!("cache miss");
                let category =
                    sqlx::query_as!(entity::Category, "select * from category where id = $1", id)
                        .fetch_one(&state.postgres)
                        .instrument(debug_span!("pg.select.*"))
                        .await
                        .map_err(map_err)?;

                // update cache
                let category = Category::from(category);

                let event = Event::UpdateSingle(Entity::Categories);

                publish_event(category.clone(), event, &self.state.jetstream).await?;
                category
            }
        };

        Ok(tonic::Response::new(category))
    }

    #[doc = " get subcategories"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn sub_categories(
        &self,
        request: tonic::Request<GetSubCategoriesRequest>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        // get cache first
        trace!("getting cache state");
        let cache = self
            .state
            .cache
            .get()
            .instrument(debug_span!("cache.get.pool"))
            .await
            .map_err(map_err)?;

        let request = request.into_inner();

        let pagination = request.pagination.expect("missing pagination params");
        let parent_id = request.id;

        // get count
        let actual_count = pagination::query_count(
            MAX_RESULTS,
            &pagination.index.ok_or_else(|| {
                tonic::Status::new(tonic::Code::Internal, "missing pagination index")
            })?,
        );
        // get 1 more
        let get_count: i64 = actual_count as i64 + 1;

        // a cursor was specified
        let (connection, cache_ok) = if let Some(ref cursor) = pagination.cursor_value {
            // get cursor
            let cursor_value = cursor.cursor_type.as_ref().ok_or_else(|| {
                tonic::Status::new(tonic::Code::Internal, "Cursor type is not set")
            })?;

            let decode_cursor = |cursor_value: &CursorType| {
                CursorBuilder::decode(cursor_value)
                    .map_err(|e| tonic::Status::internal(e.to_string()))
            };

            let connection = match cursor_value {
                CursorType::After(cursor) => {
                    // try cache first
                    let cache_key = CacheKey::Categories(CursorParams {
                        cursor: Some(cursor),
                        index: Index::First(actual_count),
                    });

                    let cache_result = read_cache(cache_key, cache).await;
                    let is_cache_ok = cache_result.is_ok();

                    let connection = if let Ok(con) = cache_result {
                        trace!("cache ok");
                        con
                    } else {
                        let cursor = decode_cursor(cursor_value)?;
                        let id = cursor.id();
                        debug!("converting to date {:?}", cursor.dt());

                        let created_at =
                            OffsetDateTime::parse(cursor.dt(), &Rfc3339).map_err(map_err)?;

                        let fut_count = sqlx::query_scalar!(
                                "
                                    select count(*) from category
                                    where 
                                        ((
                                            created_at <> $1
                                            or id <= $2
                                        )
                                        and created_at < $1) and  (($3::text is not null and parent_id = $3) or parent_id is null )
                                ",
                                created_at,
                                id,
                                parent_id
                            )
                            .fetch_one(&self.state.postgres)
                            .instrument(debug_span!("pg.select.count"))
                            .map_err(map_err);

                        let fut_categories = sqlx::query_as!(
                                entity::Category,
                                "
                                    select * from category
                                    where 
                                        ((
                                            created_at = $1
                                            and id > $2
                                        )
                                        or created_at >= $1) and  (($4::text is not null and parent_id = $4) or parent_id is null )
                                    order by
                                        created_at asc,
                                        id asc
                                    limit
                                        $3
                                ",
                                created_at,
                                id,
                                get_count,
                                parent_id
                            )
                            .fetch_all(&self.state.postgres)
                            .instrument(debug_span!("pg.select.*"))
                            .map_err(map_err);

                        let (count_on_other_end, categories) =
                            tokio::try_join!(fut_count, fut_categories)?;

                        parse_categories(count_on_other_end, categories, &pagination, actual_count)?
                    };
                    (connection, is_cache_ok)
                }
                CursorType::Before(cursor) => {
                    // try cache first
                    let cache_key = CacheKey::Categories(CursorParams {
                        cursor: Some(cursor),
                        index: Index::Last(actual_count),
                    });

                    let cache_result = read_cache(cache_key, cache).await;
                    let is_cache_ok = cache_result.is_ok();

                    let connection = if let Ok(con) = cache_result {
                        trace!("cache ok");
                        con
                    } else {
                        let cursor = decode_cursor(cursor_value)?;
                        let id = cursor.id();
                        let created_at = OffsetDateTime::parse(cursor.dt(), &Rfc3339)
                            .map_err(|e| tonic::Status::internal(e.to_string()))?;

                        let fut_count = sqlx::query_scalar!(
                                    "
                                    select count(*) from category
                                    where 
                                        ((
                                            created_at <> $1
                                            or id > $2
                                        )
                                        and created_at >= $1) and (($3::text is not null and parent_id = $3) or parent_id is null )
                                ",
                                    created_at,
                                    id,
                                    parent_id
                                )
                                .fetch_one(&self.state.postgres)
                                .instrument(debug_span!("pg.select.count"))
                                .map_err(map_err)
                            ;

                        let fut_categories = sqlx::query_as!(
                                entity::Category,
                                "
                                    select * from category
                                    where 
                                        ((
                                            created_at = $1
                                            and id < $2
                                        )
                                        or created_at < $1) and (($4::text is not null and parent_id = $4) or parent_id is null)
                                    order by
                                        created_at desc,
                                        id desc
                                    limit
                                        $3
                                ",
                                created_at,
                                id,
                                get_count,
                                parent_id
                            )
                            .fetch_all(&self.state.postgres)
                            .instrument(debug_span!("pg.select.*"))
                            .map_err(map_err);

                        let (count, categories) = tokio::try_join!(fut_count, fut_categories)?;

                        parse_categories(count, categories, &pagination, actual_count)?
                    };
                    (connection, is_cache_ok)
                }
            };
            connection
        } else {
            // try cache first
            let index = match pagination.index.expect("index to be available") {
                pagination::cursor::Index::First(count) => Index::First(count),
                pagination::cursor::Index::Last(count) => Index::Last(count),
            };
            let cache_key = CacheKey::CategoriesSubCategory(CursorParams {
                cursor: None,
                index,
            });

            let cache_result = read_cache(cache_key, cache).await;
            let is_cache_ok = cache_result.is_ok();

            let connection = if let Ok(con) = cache_result {
                trace!("cache ok");
                con
            } else {
                let categories = match index {
                    Index::First(_) => sqlx::query_as!(
                        entity::Category,
                        "select * FROM category
                            where
                                 ($2::text is not null and parent_id = $2) or parent_id is null
                            order by
                                created_at asc
                            limit $1",
                        get_count,
                        parent_id
                    )
                    .fetch_all(&self.state.postgres)
                    .instrument(debug_span!("pg.select.count"))
                    .await
                    .map_err(map_err)?,
                    Index::Last(_) => sqlx::query_as!(
                        entity::Category,
                        "select * FROM category
                            where
                                 ($2::text is not null and parent_id = $2) or parent_id is null
                            order by
                                created_at desc
                            limit $1",
                        get_count,
                        parent_id
                    )
                    .fetch_all(&self.state.postgres)
                    .instrument(debug_span!("pg.select.*"))
                    .await
                    .map_err(map_err)?,
                };

                parse_categories(
                    Some(get_count - categories.len() as i64),
                    categories,
                    &pagination,
                    actual_count,
                )?
            };
            (connection, is_cache_ok)
        };

        if !cache_ok {
            let payload = CacheCategoriesConnectionRequest {
                connection: Some(connection.clone()),
                pagination: Some(pagination),
            };

            let event = Event::UpdateBatch(Entity::Categories);

            publish_event(payload, event, &self.state.jetstream).await?;
        }

        Ok(tonic::Response::new(connection))
    }
}

#[instrument(skip(cache), err(level = Level::TRACE))]
async fn read_cache(
    cache_key: CacheKey<'_>,
    mut cache: PooledConnection<'_>,
) -> Result<Connection, tonic::Status> {
    let cache_connection = cache
        .get::<_, Vec<u8>>(&cache_key)
        .map_err(|e| tonic::Status::internal(e.to_string()))
        .and_then(|payload| async move {
            if payload.is_empty() {
                let err = "cache miss, empty bytes";
                Err(tonic::Status::internal(err))
            } else {
                CacheCategoriesConnectionRequest::decode(payload.as_ref())
                    .map_err(|e| tonic::Status::internal(e.to_string()))
            }
        })
        .await
        .map_err(map_err)?;

    cache_connection
        .connection
        .ok_or_else(|| tonic::Status::internal("corrupted cache"))
}

#[instrument(err)]
fn parse_categories(
    count_on_other_end: Option<i64>,
    categories: Vec<entity::Category>,
    pagination: &Cursor,
    actual_count: i32,
) -> Result<Connection, tonic::Status> {
    let user_count = actual_count as usize;

    let count_on_other_end = count_on_other_end
        .ok_or_else(|| tonic::Status::new(tonic::Code::Internal, "count returned no items"))?;
    let left_side = CursorBuilder::is_paginating_from_left(pagination);
    let cursor_unavailable = CursorBuilder::is_cursor_unavailable(pagination);

    let len = categories.len();

    let has_more = len > user_count;

    let to_node = |category: entity::Category| -> Result<Node, tonic::Status> {
        let category = Category::from(category);

        let dt = category
            .created_at
            .ok_or_else(|| tonic::Status::invalid_argument("timestamp is invalid"))?;

        let dt = OffsetDateTime::try_from(dt)
            .map_err(|e| tonic::Status::invalid_argument(e.to_string()))?;

        dt.to_offset(UtcOffset::UTC)
            .format(&Rfc3339)
            .map(|dt| {
                let cursor = CursorBuilder::new(&category.id, &dt);
                Node {
                    node: Some(category),
                    cursor: cursor.encode(),
                }
            })
            .map_err(map_err)
    };

    let categories: Result<Vec<_>, _> = if has_more {
        categories
            .into_iter()
            .rev() // need to take from the right hand side
            .take(user_count)
            .rev() // restore the order
            .map(&to_node)
            .collect()
    } else {
        categories.into_iter().map(&to_node).collect()
    };

    let connection = Connection {
        edges: categories?,
        page_info: Some(PageInfo {
            has_next_page: {
                if cursor_unavailable && left_side {
                    has_more
                } else {
                    count_on_other_end > 0
                }
            },
            has_previous_page: {
                if left_side {
                    count_on_other_end > 0
                } else {
                    has_more
                }
            },
            ..Default::default() // other props calculated by async-graphql
        }),
    };

    Ok(connection)
}
