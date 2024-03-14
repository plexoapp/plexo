use std::{collections::HashMap, str::FromStr, sync::Arc};

use async_graphql::dataloader::Loader;

use uuid::Uuid;

use crate::backend::engine::SDKEngine;

use super::asset::{Asset, AssetKind};

// #[derive(Clone)]
pub struct AssetLoader(Arc<SDKEngine>);

impl AssetLoader {
    pub fn new(e: Arc<SDKEngine>) -> Self {
        Self(e)
    }
}

impl Loader<Uuid> for AssetLoader {
    type Value = Asset;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &'_ [Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let assets = sqlx::query!(
            r#"
            SELECT * FROM assets WHERE id  = ANY($1)
            "#,
            &keys
        )
        .fetch_all(&*self.0.db_pool)
        .await
        .unwrap();

        //iterate to get the hashmap
        let assets_map: HashMap<Uuid, Asset> = assets
            .iter()
            .map(|asset| {
                (
                    asset.id,
                    Asset {
                        id: asset.id,
                        created_at: asset.created_at,
                        updated_at: asset.updated_at,
                        name: asset.name.clone(),
                        owner_id: asset.owner_id,
                        kind: AssetKind::from_str(&asset.kind.clone().unwrap_or_default()).unwrap_or_default(),
                        project_id: asset.project_id,
                    },
                )
            })
            .collect();

        //println!("{:?}", assets);
        Ok(assets_map)
    }
}
