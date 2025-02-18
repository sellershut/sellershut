use activitypub_federation::config::FederationConfig;
use async_graphql::{
    connection::{Connection, Edge, EmptyFields},
    Context, Object, Result,
};
use sellershut_core::{
    categories::{GetCategoryByIdRequest, GetSubCategoriesRequest},
    common::pagination::{
        cursor::{cursor_value::CursorType, CursorValue, Index},
        Cursor,
    },
};
use tonic::IntoRequest;
use tracing::{instrument, trace};

use crate::entities::category::GraphQLCategoryType as Category;
use crate::state::AppHandle;

#[derive(Default, Debug)]
pub struct CategoryGraphqlQuery;

#[Object]
impl CategoryGraphqlQuery {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn categories(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] after: Option<String>,
        #[graphql(validator(min_length = 1))] before: Option<String>,
        #[graphql(validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(validator(minimum = 1, maximum = 100))] last: Option<i32>,
    ) -> Result<Connection<String, Category, EmptyFields, EmptyFields>> {
        let pagination = Params::parse(after, before, first, last)?;

        trace!("extracting state");
        let service = ctx.data::<FederationConfig<AppHandle>>()?;

        let mut client = service.query_categories_client.clone();

        let res = client
            .categories(pagination.into_request())
            .await?
            .into_inner();

        let page_info = res.page_info.as_ref().expect("page_info to be defined");

        let mut conn = Connection::new(page_info.has_previous_page, page_info.has_next_page);

        trace!("mapping category types");

        let mut edges = Vec::with_capacity(res.edges.len());

        for edge in res.edges.into_iter() {
            let edge = Edge::new(
                edge.cursor,
                Category::try_from(edge.node.expect("category to be some"))?,
            );
            edges.push(edge);
        }
        conn.edges = edges;

        Ok(conn)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    async fn sub_categories(
        &self,
        ctx: &Context<'_>,
        parent_id: Option<String>,
        #[graphql(validator(min_length = 1, max_length = 100))] after: Option<String>,
        #[graphql(validator(min_length = 1, max_length = 100))] before: Option<String>,
        #[graphql(validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(validator(minimum = 1, maximum = 100))] last: Option<i32>,
    ) -> Result<Connection<String, Category, EmptyFields, EmptyFields>> {
        let pagination = Params::parse(after, before, first, last)?;

        trace!("extracting state");
        let service = ctx.data::<FederationConfig<AppHandle>>()?;

        let mut client = service.query_categories_client.clone();

        let req = GetSubCategoriesRequest {
            id: parent_id,
            pagination: Some(pagination),
        };

        let res = client
            .sub_categories(req.into_request())
            .await?
            .into_inner();

        let page_info = res.page_info.as_ref().expect("page_info to be defined");

        let mut conn = Connection::new(page_info.has_previous_page, page_info.has_next_page);

        let mut edges = Vec::with_capacity(res.edges.len());

        for edge in res.edges.into_iter() {
            let edge = Edge::new(
                edge.cursor,
                Category::try_from(edge.node.expect("category to be some"))?,
            );
            edges.push(edge);
        }
        conn.edges = edges;

        Ok(conn)
    }

    #[instrument(skip(ctx), err(Debug))]
    async fn category_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 21, max_length = 21))] id: String,
    ) -> async_graphql::Result<Option<Category>> {
        trace!("extracting state");
        let service = ctx.data::<FederationConfig<AppHandle>>()?;
        let request = GetCategoryByIdRequest { id };

        let mut client = service.query_categories_client.clone();
        let res = client
            .category_by_id(request.into_request())
            .await?
            .into_inner()
            .category;

        Ok(match res {
            Some(category) => Some(Category::try_from(category)?),
            None => None,
        })
    }
}

/// Relay-compliant connection parameters to page results by cursor/page size
pub struct Params;

impl Params {
    pub fn parse(
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> async_graphql::Result<Cursor> {
        trace!("parsing pagination parameters");
        if (last.is_some() && after.is_some()) || (before.is_some() && first.is_some()) {
            return Err("invalid pagination arguments. Backwards pagination needs 'last' and 'before'. Forward pagination uses 'first' and (optionally) 'after'".into());
        }
        if last.is_none() && first.is_none() {
            return Err("One of 'first' or 'last' should be provided".into());
        }

        if after.is_some() && before.is_some() {
            return Err("Only one or none of 'after' or 'before' should be provided".into());
        }

        Ok(Cursor {
            cursor_value: if after.is_some() || before.is_some() {
                Some(CursorValue {
                    cursor_type: Some(match after {
                        Some(cursor) => CursorType::After(cursor),
                        None => {
                            let before: Result<_, async_graphql::Error> =
                                before.ok_or(async_graphql::Error::new("last to be some"));
                            CursorType::Before(before?)
                        }
                    }),
                })
            } else {
                None
            },
            index: Some(if let Some(first) = first {
                Index::First(first)
            } else {
                let last: Result<_, async_graphql::Error> =
                    last.ok_or(async_graphql::Error::new("last to be some"));
                Index::Last(last?)
            }),
        })
    }
}
