use async_graphql::{Context, Object, Result};

use plexo_sdk::cognition::{
    operations::{SubdivideTaskInput, TaskSuggestion, TaskSuggestionInput},
    v2::{
        operations::CognitionOperationsV2,
        projects::{ProjectSuggestion, ProjectSuggestionInput},
    },
};

use crate::api::graphql::commons::extract_context;

#[derive(Default)]
pub struct AIProcessorGraphQLQuery;

#[derive(Default)]
pub struct AIProcessorGraphQLMutation;

#[Object]
impl AIProcessorGraphQLQuery {
    async fn suggest_next_task(&self, ctx: &Context<'_>, input: TaskSuggestionInput) -> Result<TaskSuggestion> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_suggestions_v2(input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    async fn subdivide_task(&self, ctx: &Context<'_>, input: SubdivideTaskInput) -> Result<Vec<TaskSuggestion>> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .subdivide_task_v2(input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    async fn suggest_next_project(&self, ctx: &Context<'_>, input: ProjectSuggestionInput) -> Result<ProjectSuggestion> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_project_suggestion(input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }
}
