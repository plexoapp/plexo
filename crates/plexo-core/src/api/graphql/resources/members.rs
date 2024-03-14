use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use plexo_sdk::resources::members::{member::Member as SDKMember, relations::MemberRelations};

use crate::api::graphql::commons::extract_context;

use super::{projects::Project, tasks::Task, teams::Team};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Member {
    #[graphql(flatten)]
    member: SDKMember,
}

impl From<SDKMember> for Member {
    fn from(val: SDKMember) -> Self {
        Member { member: val }
    }
}

#[ComplexObject]
impl Member {
    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<Project>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.member
            .projects(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|projects| projects.into_iter().map(|project| project.into()).collect())
    }

    async fn tasks(&self, ctx: &Context<'_>) -> Result<Vec<Task>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.member
            .tasks(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|tasks| tasks.into_iter().map(|task| task.into()).collect())
    }

    async fn teams(&self, ctx: &Context<'_>) -> Result<Vec<Team>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.member
            .teams(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|teams| teams.into_iter().map(|team| team.into()).collect())
    }
}
