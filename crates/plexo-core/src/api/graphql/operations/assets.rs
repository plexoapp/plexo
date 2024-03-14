use crate::api::graphql::{
    commons::{create_change, extract_context},
    resources::assets::Asset,
};
use async_graphql::{Context, Object, Result, Subscription};

use plexo_sdk::resources::{
    assets::operations::{AssetCrudOperations, CreateAssetInput, GetAssetsInput, UpdateAssetInput},
    changes::change::{ChangeOperation, ChangeResourceType, ListenEvent},
};
use serde_json::json;
use tokio::task;
use tokio_stream::{Stream, StreamExt};
use uuid::Uuid;

#[derive(Default)]
pub struct AssetsGraphQLQuery;

#[Object]
impl AssetsGraphQLQuery {
    async fn assets(&self, ctx: &Context<'_>, input: Option<GetAssetsInput>) -> Result<Vec<Asset>> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_assets(input.unwrap_or_default())
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|assets| assets.into_iter().map(|asset| asset.into()).collect())
    }

    async fn asset(&self, ctx: &Context<'_>, id: Uuid) -> Result<Asset> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_asset(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|asset| asset.into())
    }
}

#[derive(Default)]
pub struct AssetsGraphQLMutation;

#[Object]
impl AssetsGraphQLMutation {
    // TODO: It's possible that this method may not work correctly, as the owner_id is being ignored by async_graphql
    async fn create_asset(&self, ctx: &Context<'_>, input: CreateAssetInput) -> Result<Asset> {
        let (core, member_id) = extract_context(ctx)?;

        let mut input = input;
        input.owner_id = member_id;

        let saved_input = input.clone();

        let asset = core.engine.create_asset(input).await?;
        let saved_asset = asset.clone();

        let input = saved_input.clone();

        task::spawn(async move {
            create_change(
                &core,
                member_id,
                asset.id,
                ChangeOperation::Insert,
                ChangeResourceType::Assets,
                serde_json::to_string(&json!({
                    "input": input,
                    "result": asset,
                }))
                .unwrap(),
            )
            .await
            .unwrap();
        });

        Ok(saved_asset.into())
    }

    async fn update_asset(&self, ctx: &Context<'_>, id: Uuid, input: UpdateAssetInput) -> Result<Asset> {
        let (core, member_id) = extract_context(ctx)?;

        let saved_input = input.clone();

        let asset = core.engine.update_asset(id, input).await?;

        let asset = asset.clone();
        let saved_asset = asset.clone();

        tokio::spawn(async move {
            create_change(
                &core,
                member_id,
                asset.id,
                ChangeOperation::Update,
                ChangeResourceType::Assets,
                serde_json::to_string(&json!({
                    "input": saved_input,
                    "result": asset,
                }))
                .unwrap(),
            )
            .await
            .unwrap();
        });

        Ok(saved_asset.into())
    }

    async fn delete_asset(&self, ctx: &Context<'_>, id: Uuid) -> Result<Asset> {
        let (core, _member_id) = extract_context(ctx)?;

        let asset = core.engine.delete_asset(id).await?;
        let saved_asset = asset.clone();

        tokio::spawn(async move {
            create_change(
                &core,
                asset.owner_id,
                asset.id,
                ChangeOperation::Delete,
                ChangeResourceType::Assets,
                serde_json::to_string(&json!({
                    "result": asset,
                }))
                .unwrap(),
            )
            .await
            .unwrap();
        });

        Ok(saved_asset.into())
    }
}

#[derive(Default)]
pub struct AssetsGraphQLSubscription;

#[Subscription]
impl AssetsGraphQLSubscription {
    async fn assets(&self, ctx: &Context<'_>) -> impl Stream<Item = ListenEvent> {
        let (core, _member_id) = extract_context(ctx).unwrap();

        core.engine
            .listen(ChangeResourceType::Assets)
            .await
            .unwrap()
            .map(|x| x.unwrap())
    }
}
