use async_graphql::{Context, Object, Result, Subscription};

use plexo_sdk::{
    cognition::{
        operations::{SubdivideTaskInput, TaskSuggestion, TaskSuggestionInput},
        v2::{
            operations::CognitionOperationsV2,
            projects::{ProjectSuggestion, ProjectSuggestionInput},
        },
    },
    resources::chats::{
        chat::Chat,
        operations::{ChatCrudOperations, CreateChatInput},
    },
};

use tokio_stream::Stream;
use uuid::Uuid;

use crate::api::graphql::commons::extract_context;

#[derive(Default)]
pub struct AIProcessorGraphQLQuery;

#[derive(Default)]
pub struct AIProcessorGraphQLMutation;

#[derive(Default)]
pub struct AIProcessorGraphQLSubscription;

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

    async fn suggest_next_project(
        &self,
        ctx: &Context<'_>,
        input: ProjectSuggestionInput,
    ) -> Result<ProjectSuggestion> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_project_suggestion(input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }
}

#[Object]
impl AIProcessorGraphQLMutation {
    async fn create_chat(&self, ctx: &Context<'_>, input: CreateChatInput) -> Result<Chat> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .create_chat(input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }
}

#[Subscription]
impl AIProcessorGraphQLSubscription {
    async fn chat(&self, ctx: &Context<'_>, chat_id: Uuid, _message: String) -> impl Stream<Item = u8> {
        let (core, _member_id) = extract_context(ctx).unwrap();

        let _chat = core.engine.get_chat(chat_id).await.unwrap();

        // match chat.resource_type {
        //     "project" => {},
        //     _ => {},
        // }

        tokio_stream::iter(vec![127])
    }
}
