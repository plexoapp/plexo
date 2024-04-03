use crate::api::graphql::{
    commons::{create_change, extract_context},
    resources::messages::Message,
};
use async_graphql::{Context, Object, Result, Subscription};

use plexo_sdk::resources::{
    changes::change::{ChangeOperation, ChangeResourceType, ListenEvent},
    messages::operations::{CreateMessageInput, GetMessagesInput, MessageCrudOperations, UpdateMessageInput},
};

use serde_json::json;
use tokio::task;
use tokio_stream::{Stream, StreamExt};
use uuid::Uuid;

#[derive(Default)]
pub struct MessagesGraphQLQuery;

#[Object]
impl MessagesGraphQLQuery {
    async fn messages(&self, ctx: &Context<'_>, input: Option<GetMessagesInput>) -> Result<Vec<Message>> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_messages(input.unwrap_or_default())
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|messages| messages.into_iter().map(|message| message.into()).collect())
    }

    async fn message(&self, ctx: &Context<'_>, id: Uuid) -> Result<Message> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_message(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|message| message.into())
    }
}

#[derive(Default)]
pub struct MessagesGraphQLMutation;

#[Object]
impl MessagesGraphQLMutation {
    // TODO: It's possible that this method may not work correctly, as the owner_id is being ignored by async_graphql
    async fn create_message(&self, ctx: &Context<'_>, input: CreateMessageInput) -> Result<Message> {
        let (core, member_id) = extract_context(ctx)?;

        let mut input = input;
        input.owner_id = member_id;

        let saved_input = input.clone();

        let message = core.engine.create_message(input).await?;
        let saved_message = message.clone();

        let input = saved_input.clone();

        task::spawn(async move {
            create_change(
                &core,
                member_id,
                message.id,
                ChangeOperation::Insert,
                ChangeResourceType::Messages,
                serde_json::to_string(&json!({
                    "input": input,
                    "result": message,
                }))
                .unwrap(),
            )
            .await
            .unwrap();
        });

        Ok(saved_message.into())
    }

    async fn update_message(&self, ctx: &Context<'_>, id: Uuid, input: UpdateMessageInput) -> Result<Message> {
        let (core, member_id) = extract_context(ctx)?;

        let saved_input = input.clone();

        let message = core.engine.update_message(id, input).await?;

        let message = message.clone();
        let saved_message = message.clone();

        tokio::spawn(async move {
            create_change(
                &core,
                member_id,
                message.id,
                ChangeOperation::Update,
                ChangeResourceType::Messages,
                serde_json::to_string(&json!({
                    "input": saved_input,
                    "result": message,
                }))
                .unwrap(),
            )
            .await
            .unwrap();
        });

        Ok(saved_message.into())
    }

    async fn delete_message(&self, ctx: &Context<'_>, id: Uuid) -> Result<Message> {
        let (core, _member_id) = extract_context(ctx)?;

        let message = core.engine.delete_message(id).await?;
        let saved_message = message.clone();

        tokio::spawn(async move {
            create_change(
                &core,
                message.owner_id,
                message.id,
                ChangeOperation::Delete,
                ChangeResourceType::Messages,
                serde_json::to_string(&json!({
                    "result": message,
                }))
                .unwrap(),
            )
            .await
            .unwrap();
        });

        Ok(saved_message.into())
    }
}

#[derive(Default)]
pub struct MessagesGraphQLSubscription;

#[Subscription]
impl MessagesGraphQLSubscription {
    async fn messages(&self, ctx: &Context<'_>) -> impl Stream<Item = ListenEvent> {
        let (core, _member_id) = extract_context(ctx).unwrap();

        core.engine
            .listen(ChangeResourceType::Messages)
            .await
            .unwrap()
            .map(|x| x.unwrap())
    }
}
