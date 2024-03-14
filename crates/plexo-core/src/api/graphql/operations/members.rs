use crate::api::graphql::{commons::extract_context, resources::members::Member};
use async_graphql::{Context, Object, Result, Subscription};

use plexo_sdk::resources::{
    changes::change::{ChangeResourceType, ListenEvent},
    members::operations::{CreateMemberInput, GetMembersInput, MemberCrudOperations, UpdateMemberInput},
};
use tokio_stream::{Stream, StreamExt};
use uuid::Uuid;

#[derive(Default)]
pub struct MembersGraphQLQuery;

#[Object]
impl MembersGraphQLQuery {
    async fn members(&self, ctx: &Context<'_>, input: Option<GetMembersInput>) -> Result<Vec<Member>> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_members(input.unwrap_or_default())
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|members| members.into_iter().map(|member| member.into()).collect())
    }

    async fn member(&self, ctx: &Context<'_>, id: Uuid) -> Result<Member> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_member(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|member| member.into())
    }
}

#[derive(Default)]
pub struct MembersGraphQLMutation;

#[Object]
impl MembersGraphQLMutation {
    async fn create_member(&self, ctx: &Context<'_>, input: CreateMemberInput) -> Result<Member> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .create_member(input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|member| member.into())
    }

    async fn update_member(&self, ctx: &Context<'_>, id: Uuid, input: UpdateMemberInput) -> Result<Member> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .update_member(id, input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|member| member.into())
    }

    async fn delete_member(&self, ctx: &Context<'_>, id: Uuid) -> Result<Member> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .delete_member(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|member| member.into())
    }
}

#[derive(Default)]
pub struct MembersGraphQLSubscription;

#[Subscription]
impl MembersGraphQLSubscription {
    async fn members(&self, ctx: &Context<'_>) -> impl Stream<Item = ListenEvent> {
        let (core, _member_id) = extract_context(ctx).unwrap();

        core.engine
            .listen(ChangeResourceType::Members)
            .await
            .unwrap()
            .map(|x| x.unwrap())
    }
}
