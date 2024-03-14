use crate::api::graphql::{
    commons::{create_change, extract_context},
    resources::teams::Team,
};
use async_graphql::{Context, Object, Result, Subscription};

use plexo_sdk::resources::{
    changes::change::{ChangeOperation, ChangeResourceType, ListenEvent},
    teams::operations::{CreateTeamInput, GetTeamsInput, TeamCrudOperations, UpdateTeamInput},
};

use serde_json::json;
use tokio::task;
use tokio_stream::{Stream, StreamExt};
use uuid::Uuid;

#[derive(Default)]
pub struct TeamsGraphQLQuery;

#[Object]
impl TeamsGraphQLQuery {
    async fn teams(&self, ctx: &Context<'_>, input: Option<GetTeamsInput>) -> Result<Vec<Team>> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_teams(input.unwrap_or_default())
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|teams| teams.into_iter().map(|team| team.into()).collect())
    }

    async fn team(&self, ctx: &Context<'_>, id: Uuid) -> Result<Team> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_team(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|team| team.into())
    }
}

#[derive(Default)]
pub struct TeamsGraphQLMutation;

#[Object]
impl TeamsGraphQLMutation {
    // TODO: It's possible that this method may not work correctly, as the owner_id is being ignored by async_graphql
    async fn create_team(&self, ctx: &Context<'_>, input: CreateTeamInput) -> Result<Team> {
        let (core, member_id) = extract_context(ctx)?;

        let mut input = input;
        input.owner_id = member_id;

        let saved_input = input.clone();

        let team = core.engine.create_team(input).await?;
        let saved_team = team.clone();

        let input = saved_input.clone();

        task::spawn(async move {
            create_change(
                &core,
                member_id,
                team.id,
                ChangeOperation::Insert,
                ChangeResourceType::Teams,
                serde_json::to_string(&json!({
                    "input": input,
                    "result": team,
                }))
                .unwrap(),
            )
            .await
            .unwrap();
        });

        Ok(saved_team.into())

        // let (core, member_id) = extract_context(ctx)?;

        // let mut input = input;
        // input.owner_id = member_id;

        // core.engine
        //     .create_team(input)
        //     .await
        //     .map_err(|err| async_graphql::Error::new(err.to_string()))
        //     .map(|team| team.into())
    }

    async fn update_team(&self, ctx: &Context<'_>, id: Uuid, input: UpdateTeamInput) -> Result<Team> {
        let (core, member_id) = extract_context(ctx)?;

        let saved_input = input.clone();

        let team = core.engine.update_team(id, input).await?;

        let team = team.clone();
        let saved_team = team.clone();

        tokio::spawn(async move {
            create_change(
                &core,
                member_id,
                team.id,
                ChangeOperation::Update,
                ChangeResourceType::Teams,
                serde_json::to_string(&json!({
                    "input": saved_input,
                    "result": team,
                }))
                .unwrap(),
            )
            .await
            .unwrap();
        });

        Ok(saved_team.into())

        // core.engine
        //     .update_team(id, input)
        //     .await
        //     .map_err(|err| async_graphql::Error::new(err.to_string()))
        //     .map(|team| team.into())
    }

    async fn delete_team(&self, ctx: &Context<'_>, id: Uuid) -> Result<Team> {
        let (core, _member_id) = extract_context(ctx)?;

        let team = core.engine.delete_team(id).await?;
        let saved_team = team.clone();

        tokio::spawn(async move {
            create_change(
                &core,
                team.owner_id,
                team.id,
                ChangeOperation::Delete,
                ChangeResourceType::Teams,
                serde_json::to_string(&json!({
                    "result": team,
                }))
                .unwrap(),
            )
            .await
            .unwrap();
        });

        Ok(saved_team.into())

        // core.engine
        //     .delete_team(id)
        //     .await
        //     .map_err(|err| async_graphql::Error::new(err.to_string()))
        //     .map(|team| team.into())
    }
}

#[derive(Default)]
pub struct TeamsGraphQLSubscription;

#[Subscription]
impl TeamsGraphQLSubscription {
    async fn teams(&self, ctx: &Context<'_>) -> impl Stream<Item = ListenEvent> {
        let (core, _member_id) = extract_context(ctx).unwrap();

        core.engine
            .listen(ChangeResourceType::Teams)
            .await
            .unwrap()
            .map(|x| x.unwrap())
    }
}
