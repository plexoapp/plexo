use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use plexo_sdk::resources::tasks::{relations::TaskRelations, task::Task as SDKTask};

use crate::api::graphql::commons::extract_context;

use super::{changes::Change, labels::Label, members::Member, projects::Project};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Task {
    #[graphql(flatten)]
    task: SDKTask,
}

impl From<SDKTask> for Task {
    fn from(val: SDKTask) -> Self {
        Task { task: val }
    }
}

#[ComplexObject]
impl Task {
    async fn owner(&self, ctx: &Context<'_>) -> Result<Member> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.task
            .owner(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|member| member.into())
    }

    async fn project(&self, ctx: &Context<'_>) -> Result<Option<Project>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.task
            .project(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|project| project.map(|p| p.into()))
    }

    async fn lead(&self, ctx: &Context<'_>) -> Result<Option<Member>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.task
            .lead(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|lead| lead.map(|l| l.into()))
    }

    async fn parent(&self, ctx: &Context<'_>) -> Result<Option<Task>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.task
            .parent(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|task| task.map(|t| t.into()))
    }

    async fn assignees(&self, ctx: &Context<'_>) -> Result<Vec<Member>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.task
            .assignees(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|members| members.into_iter().map(|member| member.into()).collect())
    }

    async fn labels(&self, ctx: &Context<'_>) -> Result<Vec<Label>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.task
            .labels(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|labels| labels.into_iter().map(|label| label.into()).collect())
    }

    async fn subtasks(&self, ctx: &Context<'_>) -> Result<Vec<Task>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.task
            .subtasks(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|tasks| tasks.into_iter().map(|task| task.into()).collect())
    }

    async fn changes(&self, ctx: &Context<'_>) -> Result<Vec<Change>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.task
            .changes(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|changes| changes.into_iter().map(|change| change.into()).collect())
    }
}
