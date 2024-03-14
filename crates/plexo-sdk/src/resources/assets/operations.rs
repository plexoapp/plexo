use std::str::FromStr;

use async_graphql::InputObject;
use async_trait::async_trait;
use derive_builder::Builder;
use poem_openapi::Object;
use serde::Serialize;
use sqlx::Row;
use uuid::Uuid;

use crate::backend::engine::SDKEngine;
use crate::common::commons::SortOrder;
use crate::errors::sdk::SDKError;
use crate::resources::assets::asset::{Asset, AssetKind};

#[async_trait]
pub trait AssetCrudOperations {
    async fn create_asset(&self, input: CreateAssetInput) -> Result<Asset, SDKError>;
    async fn get_asset(&self, id: Uuid) -> Result<Asset, SDKError>;
    async fn get_assets(&self, input: GetAssetsInput) -> Result<Vec<Asset>, SDKError>;
    async fn update_asset(&self, id: Uuid, input: UpdateAssetInput) -> Result<Asset, SDKError>;
    async fn delete_asset(&self, id: Uuid) -> Result<Asset, SDKError>;
}

#[derive(Clone, Default, Builder, Object, InputObject, Serialize)]
#[builder(pattern = "owned")]
pub struct CreateAssetInput {
    pub name: String,

    #[graphql(skip)]
    pub owner_id: Uuid,

    #[builder(setter(strip_option), default)]
    pub kind: Option<AssetKind>,
    #[builder(setter(strip_option), default)]
    pub project_id: Option<Uuid>,
}

#[derive(Clone, Default, Builder, Object, InputObject, Serialize)]
#[builder(pattern = "owned")]
pub struct UpdateAssetInput {
    #[builder(setter(strip_option), default)]
    pub name: Option<String>,
    #[builder(setter(strip_option), default)]
    pub kind: Option<AssetKind>,
    #[builder(setter(strip_option), default)]
    pub project_id: Option<Uuid>,
}

#[derive(Default, Builder, Object, InputObject)]
#[builder(pattern = "owned")]
pub struct GetAssetsInput {
    #[builder(setter(strip_option), default)]
    pub filter: Option<GetAssetsWhere>,

    #[builder(setter(strip_option), default)]
    pub sort_by: Option<String>,
    #[builder(setter(strip_option), default)]
    pub sort_order: Option<SortOrder>,

    #[builder(setter(into, strip_option), default = "Some(100)")]
    pub limit: Option<i32>,
    #[builder(setter(into, strip_option), default = "Some(0)")]
    pub offset: Option<i32>,
}

#[derive(Default, Builder, Object, InputObject)]
#[builder(pattern = "owned")]
pub struct GetAssetsWhere {
    #[builder(setter(strip_option), default)]
    pub ids: Option<Vec<Uuid>>,
    #[builder(setter(strip_option), default)]
    pub owner_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub name: Option<String>,
    #[builder(setter(strip_option), default)]
    pub kind: Option<AssetKind>,
    #[builder(setter(strip_option), default)]
    pub project_id: Option<Uuid>,

    #[oai(skip)]
    #[builder(setter(strip_option), default)]
    pub _and: Option<Vec<GetAssetsWhere>>,
    #[oai(skip)]
    #[builder(setter(strip_option), default)]
    pub _or: Option<Vec<GetAssetsWhere>>,
}

impl GetAssetsWhere {
    pub fn compile_sql(&self) -> String {
        let mut where_clause = Vec::new();

        if let Some(ids) = &self.ids {
            where_clause.push(format!(
                "id = ANY(array[{}]::uuid[])",
                ids.iter()
                    .map(|x| format!("'{}'", x))
                    .collect::<Vec<String>>()
                    .join(",")
            ));
        }

        if let Some(name) = &self.name {
            where_clause.push(format!("name = '{}'", name));
        }

        if let Some(kind) = &self.kind {
            where_clause.push(format!("kind = '{}'", kind));
        }

        if let Some(project_id) = &self.project_id {
            where_clause.push(format!("project_id = '{}'", project_id));
        }

        if let Some(owner_id) = &self.owner_id {
            where_clause.push(format!("owner_id = '{}'", owner_id));
        }

        if let Some(_and) = &self._and {
            where_clause.push(format!(
                "({})",
                _and.iter()
                    .map(|x| x.compile_sql())
                    .collect::<Vec<String>>()
                    .join(" AND ")
            ));
        }

        if let Some(_or) = &self._or {
            where_clause.push(format!(
                "({})",
                _or.iter()
                    .map(|x| x.compile_sql())
                    .collect::<Vec<String>>()
                    .join(" OR ")
            ));
        }

        where_clause.join(" AND ")
    }
}

#[async_trait]
impl AssetCrudOperations for SDKEngine {
    async fn create_asset(&self, input: CreateAssetInput) -> Result<Asset, SDKError> {
        let asset_final_info = sqlx::query!(
            r#"
            INSERT INTO assets (name, owner_id, kind, project_id)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            input.name,
            input.owner_id,
            input.kind.map(|k| k.to_string()),
            input.project_id,
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        Ok(Asset {
            id: asset_final_info.id,
            created_at: asset_final_info.created_at,
            updated_at: asset_final_info.updated_at,
            name: asset_final_info.name,
            owner_id: asset_final_info.owner_id,
            kind: AssetKind::from_str(&asset_final_info.kind.unwrap_or_default()).unwrap_or_default(),
            project_id: asset_final_info.project_id,
        })
    }

    async fn get_asset(&self, id: Uuid) -> Result<Asset, SDKError> {
        let asset_info = sqlx::query!(
            r#"
            SELECT * FROM assets WHERE id = $1
            "#,
            id,
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        Ok(Asset {
            id: asset_info.id,
            created_at: asset_info.created_at,
            updated_at: asset_info.updated_at,
            name: asset_info.name,
            owner_id: asset_info.owner_id,
            kind: AssetKind::from_str(&asset_info.kind.unwrap_or_default()).unwrap_or_default(),
            project_id: asset_info.project_id,
        })
    }

    async fn get_assets(&self, input: GetAssetsInput) -> Result<Vec<Asset>, SDKError> {
        let mut query = "SELECT * FROM assets ".to_string();

        if let Some(filter) = input.filter {
            query.push_str(&format!("WHERE {} ", filter.compile_sql()));
        }

        if let Some(sort_by) = input.sort_by {
            query.push_str(&format!("ORDER BY {} ", sort_by));
        }

        if let Some(sort_order) = input.sort_order {
            query.push_str(&format!("{} ", sort_order));
        }

        if let Some(limit) = input.limit {
            query.push_str(&format!("LIMIT {} ", limit));
        }

        if let Some(offset) = input.offset {
            query.push_str(&format!("OFFSET {} ", offset));
        }

        let assets_info = sqlx::query(query.as_str()).fetch_all(self.db_pool.as_ref()).await?;

        let assets = assets_info
            .into_iter()
            .map(|asset_info| Asset {
                id: asset_info.get("id"),
                created_at: asset_info.get("created_at"),
                updated_at: asset_info.get("updated_at"),
                name: asset_info.get("name"),
                owner_id: asset_info.get("owner_id"),
                kind: asset_info
                    .get::<'_, Option<String>, _>("kind")
                    .and_then(|a| AssetKind::from_str(&a).ok())
                    .unwrap_or_default(),
                project_id: asset_info.get("project_id"),
            })
            .collect::<Vec<Asset>>();

        Ok(assets)
    }

    async fn update_asset(&self, id: Uuid, input: UpdateAssetInput) -> Result<Asset, SDKError> {
        let asset_final_info = sqlx::query!(
            r#"
            UPDATE assets
            SET
                name = COALESCE($1, name),
                kind = COALESCE($2, kind),
                project_id = COALESCE($3, project_id)
            WHERE id = $4
            RETURNING *
            "#,
            input.name,
            input.kind.map(|k| k.to_string()),
            input.project_id,
            id,
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        Ok(Asset {
            id: asset_final_info.id,
            created_at: asset_final_info.created_at,
            updated_at: asset_final_info.updated_at,
            name: asset_final_info.name,
            owner_id: asset_final_info.owner_id,
            kind: AssetKind::from_str(&asset_final_info.kind.unwrap_or_default()).unwrap_or_default(),
            project_id: asset_final_info.project_id,
        })
    }

    async fn delete_asset(&self, id: Uuid) -> Result<Asset, SDKError> {
        let asset_info = sqlx::query!(
            r#"
            DELETE FROM assets WHERE id = $1
            RETURNING *
            "#,
            id,
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        Ok(Asset {
            id: asset_info.id,
            created_at: asset_info.created_at,
            updated_at: asset_info.updated_at,
            name: asset_info.name,
            owner_id: asset_info.owner_id,
            kind: AssetKind::from_str(&asset_info.kind.unwrap_or_default()).unwrap_or_default(),
            project_id: asset_info.project_id,
        })
    }
}
