use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use plexo_sdk::resources::assets::{asset::Asset as SDKAsset, relations::AssetRelations};

use crate::api::graphql::commons::extract_context;

use super::{members::Member, projects::Project};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Asset {
    #[graphql(flatten)]
    asset: SDKAsset,
}

impl From<SDKAsset> for Asset {
    fn from(val: SDKAsset) -> Self {
        Asset { asset: val }
    }
}

#[ComplexObject]
impl Asset {
    async fn owner(&self, ctx: &Context<'_>) -> Result<Member> {
        let (plexo_engine, _asset_id) = extract_context(ctx)?;

        self.asset
            .owner(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|asset| asset.into())
    }

    async fn project(&self, ctx: &Context<'_>) -> Result<Option<Project>> {
        let (plexo_engine, _asset_id) = extract_context(ctx)?;

        self.asset
            .project(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|project| project.map(|project| project.into()))
    }
}
