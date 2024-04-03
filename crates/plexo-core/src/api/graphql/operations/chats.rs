use crate::api::graphql::{
    commons::{create_change, extract_context},
    resources::chats::Chat,
};
use async_graphql::{Context, Object, Result, Subscription};

use plexo_sdk::resources::{
    changes::change::{ChangeOperation, ChangeResourceType, ListenEvent},
    chats::operations::{ChatCrudOperations, CreateChatInput, GetChatsInput, UpdateChatInput},
};

use serde_json::json;
use tokio::task;
use tokio_stream::{Stream, StreamExt};
use uuid::Uuid;

#[derive(Default)]
pub struct ChatsGraphQLQuery;

#[Object]
impl ChatsGraphQLQuery {
    async fn chats(&self, ctx: &Context<'_>, input: Option<GetChatsInput>) -> Result<Vec<Chat>> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_chats(input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|chats| chats.into_iter().map(|chat| chat.into()).collect())
    }

    async fn chat(&self, ctx: &Context<'_>, id: Uuid) -> Result<Chat> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_chat(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|chat| chat.into())
    }
}

#[derive(Default)]
pub struct ChatsGraphQLMutation;

#[Object]
impl ChatsGraphQLMutation {
    // TODO: It's possible that this method may not work correctly, as the owner_id is being ignored by async_graphql
    async fn create_chat(&self, ctx: &Context<'_>, input: CreateChatInput) -> Result<Chat> {
        let (core, member_id) = extract_context(ctx)?;

        let mut input = input;
        input.owner_id = member_id;

        let saved_input = input.clone();

        let chat = core.engine.create_chat(input).await?;
        let saved_chat = chat.clone();

        let input = saved_input.clone();

        task::spawn(async move {
            create_change(
                &core,
                member_id,
                chat.id,
                ChangeOperation::Insert,
                ChangeResourceType::Chats,
                serde_json::to_string(&json!({
                    "input": input,
                    "result": chat,
                }))
                .unwrap(),
            )
            .await
            .unwrap();
        });

        Ok(saved_chat.into())
    }

    async fn update_chat(&self, ctx: &Context<'_>, id: Uuid, input: UpdateChatInput) -> Result<Chat> {
        let (core, member_id) = extract_context(ctx)?;

        let saved_input = input.clone();

        let chat = core.engine.update_chat(id, input).await?;

        let chat = chat.clone();
        let saved_chat = chat.clone();

        tokio::spawn(async move {
            create_change(
                &core,
                member_id,
                chat.id,
                ChangeOperation::Update,
                ChangeResourceType::Chats,
                serde_json::to_string(&json!({
                    "input": saved_input,
                    "result": chat,
                }))
                .unwrap(),
            )
            .await
            .unwrap();
        });

        Ok(saved_chat.into())
    }

    async fn delete_chat(&self, ctx: &Context<'_>, id: Uuid) -> Result<Chat> {
        let (core, _member_id) = extract_context(ctx)?;

        let chat = core.engine.delete_chat(id).await?;
        let saved_chat = chat.clone();

        tokio::spawn(async move {
            create_change(
                &core,
                chat.owner_id,
                chat.id,
                ChangeOperation::Delete,
                ChangeResourceType::Chats,
                serde_json::to_string(&json!({
                    "result": chat,
                }))
                .unwrap(),
            )
            .await
            .unwrap();
        });

        Ok(saved_chat.into())
    }
}

#[derive(Default)]
pub struct ChatsGraphQLSubscription;

#[Subscription]
impl ChatsGraphQLSubscription {
    async fn chats(&self, ctx: &Context<'_>) -> impl Stream<Item = ListenEvent> {
        let (core, _member_id) = extract_context(ctx).unwrap();

        core.engine
            .listen(ChangeResourceType::Chats)
            .await
            .unwrap()
            .map(|x| x.unwrap())
    }
}
