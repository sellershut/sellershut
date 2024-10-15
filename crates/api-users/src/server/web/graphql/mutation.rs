use anyhow::Context as _Ctx;
use async_graphql::{Context, MergedObject, Object};
use sellershut_core::users::{mutate_users_server::MutateUsers, CreateUserRequest};
use tonic::IntoRequest;
use tracing::instrument;

use crate::{entity::User, state::AppState};

#[derive(Default, Debug, MergedObject)]
pub struct Mutation(GraphqlMutation);

#[derive(Default, Debug)]
pub struct GraphqlMutation;

#[Object]
impl GraphqlMutation {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn create(&self, ctx: &Context<'_>, input: User) -> async_graphql::Result<User> {
        let database = ctx.data::<AppState>()?;

        let user = sellershut_core::users::User::from(input);

        let request = CreateUserRequest { user: Some(user) };

        let result = database
            .create_user(request.into_request())
            .await?
            .into_inner()
            .user
            .context("user to be created")?;

        Ok(User::try_from(result)?)
    }
}
