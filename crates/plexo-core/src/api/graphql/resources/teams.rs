use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use plexo_sdk::resources::teams::{relations::TeamRelations, team::Team as SDKTeam};

use crate::api::graphql::commons::extract_context;

use super::members::Member;
use super::projects::Project;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Team {
    #[graphql(flatten)]
    team: SDKTeam,
}

impl From<SDKTeam> for Team {
    fn from(val: SDKTeam) -> Self {
        Team { team: val }
    }
}

#[ComplexObject]
impl Team {
    async fn owner(&self, ctx: &Context<'_>) -> Result<Member> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.team
            .owner(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|member| member.into())
    }

    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<Project>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.team
            .projects(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|projects| projects.into_iter().map(|project| project.into()).collect())
    }

    async fn members(&self, ctx: &Context<'_>) -> Result<Vec<Member>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.team
            .members(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|members| members.into_iter().map(|member| member.into()).collect())
    }
}
