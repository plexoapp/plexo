use crate::api::graphql::{
    commons::{create_change, extract_context},
    resources::labels::Label,
};
use async_graphql::{Context, Object, Result, Subscription};

use plexo_sdk::resources::{
    changes::change::{ChangeOperation, ChangeResourceType, ListenEvent},
    labels::operations::{CreateLabelInput, GetLabelsInput, LabelCrudOperations, UpdateLabelInput},
};
use serde_json::json;
use tokio::task;
use tokio_stream::{Stream, StreamExt};
use uuid::Uuid;

#[derive(Default)]
pub struct LabelsGraphQLQuery;

#[Object]
impl LabelsGraphQLQuery {
    async fn labels(&self, ctx: &Context<'_>, input: Option<GetLabelsInput>) -> Result<Vec<Label>> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_labels(input.unwrap_or_default())
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|labels| labels.into_iter().map(|label| label.into()).collect())
    }

    async fn label(&self, ctx: &Context<'_>, id: Uuid) -> Result<Label> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_label(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|label| label.into())
    }
}

#[derive(Default)]
pub struct LabelsGraphQLMutation;

#[Object]
impl LabelsGraphQLMutation {
    // TODO: It's possible that this method may not work correctly, as the owner_id is being ignored by async_graphql
    async fn create_label(&self, ctx: &Context<'_>, input: CreateLabelInput) -> Result<Label> {
        let (core, member_id) = extract_context(ctx)?;

        let mut input = input;
        input.owner_id = member_id;

        let saved_input = input.clone();

        let label = core.engine.create_label(input).await?;
        let saved_label = label.clone();

        let input = saved_input.clone();

        task::spawn(async move {
            create_change(
                &core,
                member_id,
                label.id,
                ChangeOperation::Insert,
                ChangeResourceType::Labels,
                serde_json::to_string(&json!({
                    "input": input,
                    "result": label,
                }))
                .unwrap(),
            )
            .await
            .unwrap();
        });

        Ok(saved_label.into())
    }

    async fn update_label(&self, ctx: &Context<'_>, id: Uuid, input: UpdateLabelInput) -> Result<Label> {
        let (core, member_id) = extract_context(ctx)?;

        let saved_input = input.clone();

        let label = core.engine.update_label(id, input).await?;

        let label = label.clone();
        let saved_label = label.clone();

        tokio::spawn(async move {
            create_change(
                &core,
                member_id,
                label.id,
                ChangeOperation::Update,
                ChangeResourceType::Labels,
                serde_json::to_string(&json!({
                    "input": saved_input,
                    "result": label,
                }))
                .unwrap(),
            )
            .await
            .unwrap();
        });

        Ok(saved_label.into())
    }

    async fn delete_label(&self, ctx: &Context<'_>, id: Uuid) -> Result<Label> {
        let (core, _member_id) = extract_context(ctx)?;

        let label = core.engine.delete_label(id).await?;
        let saved_label = label.clone();

        tokio::spawn(async move {
            create_change(
                &core,
                label.owner_id,
                label.id,
                ChangeOperation::Delete,
                ChangeResourceType::Labels,
                serde_json::to_string(&json!({
                    "result": label,
                }))
                .unwrap(),
            )
            .await
            .unwrap();
        });

        Ok(saved_label.into())
    }
}

#[derive(Default)]
pub struct LabelsGraphQLSubscription;

#[Subscription]
impl LabelsGraphQLSubscription {
    async fn labels(&self, ctx: &Context<'_>) -> impl Stream<Item = ListenEvent> {
        let (core, _member_id) = extract_context(ctx).unwrap();

        core.engine
            .listen(ChangeResourceType::Labels)
            .await
            .unwrap()
            .map(|x| x.unwrap())
    }
}
