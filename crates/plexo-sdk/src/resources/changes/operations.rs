use std::str::FromStr;

use async_graphql::InputObject;
use async_trait::async_trait;
use derive_builder::Builder;
use poem_openapi::Object;
use serde::Serialize;
use sqlx::Row;
use uuid::Uuid;

use crate::{backend::engine::SDKEngine, common::commons::SortOrder, errors::sdk::SDKError};

use super::change::{Change, ChangeOperation, ChangeResourceType};

#[async_trait]
pub trait ChangeCrudOperations {
    async fn create_change(&self, input: CreateChangeInput) -> Result<Change, SDKError>;
    async fn get_change(&self, id: Uuid) -> Result<Change, SDKError>;
    async fn get_changes(&self, input: GetChangesInput) -> Result<Vec<Change>, SDKError>;
    async fn update_change(&self, id: Uuid, input: UpdateChangeInput) -> Result<Change, SDKError>;
    async fn delete_change(&self, id: Uuid) -> Result<Change, SDKError>;
}

#[derive(Clone, Builder, Object, InputObject)]
#[builder(pattern = "owned")]
pub struct CreateChangeInput {
    #[graphql(skip)]
    pub owner_id: Uuid,
    pub resource_id: Uuid,

    pub operation: ChangeOperation,
    pub resource_type: ChangeResourceType,

    pub diff_json: String,
}

#[derive(Clone, Default, Builder, Object, InputObject, Serialize)]
#[builder(pattern = "owned")]
pub struct UpdateChangeInput {
    #[builder(setter(strip_option), default)]
    pub owner_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub resource_id: Option<Uuid>,

    #[builder(setter(strip_option), default)]
    pub operation: Option<ChangeOperation>,
    #[builder(setter(strip_option), default)]
    pub resource_type: Option<ChangeResourceType>,

    #[builder(setter(strip_option), default)]
    pub diff_json: Option<String>,
}

#[derive(Default, Builder, Object, InputObject)]
#[builder(pattern = "owned")]
pub struct GetChangesInput {
    #[builder(setter(strip_option), default)]
    pub filter: Option<GetChangesWhere>,

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
pub struct GetChangesWhere {
    #[builder(setter(strip_option), default)]
    pub ids: Option<Vec<Uuid>>,
    #[builder(setter(strip_option), default)]
    pub owner_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub resource_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub operation: Option<ChangeOperation>,
    #[builder(setter(strip_option), default)]
    pub resource_type: Option<ChangeResourceType>,

    #[oai(skip)]
    #[builder(setter(strip_option), default)]
    pub _and: Option<Vec<GetChangesWhere>>,
    #[oai(skip)]
    #[builder(setter(strip_option), default)]
    pub _or: Option<Vec<GetChangesWhere>>,
}

impl GetChangesWhere {
    pub fn compile_sql(&self) -> String {
        let mut and_clauses = Vec::new();
        let mut or_clauses = Vec::new();

        if let Some(ids) = &self.ids {
            and_clauses.push(format!(
                "id = ANY(array[{}]::uuid[])",
                ids.iter()
                    .map(|x| format!("'{}'", x))
                    .collect::<Vec<String>>()
                    .join(",")
            ));
        }

        if let Some(owner_id) = &self.owner_id {
            and_clauses.push(format!("owner_id = '{}'", owner_id));
        }
        if let Some(resource_id) = &self.resource_id {
            and_clauses.push(format!("resource_id = '{}'", resource_id));
        }
        if let Some(operation) = &self.operation {
            and_clauses.push(format!("operation = '{}'", operation));
        }
        if let Some(resource_type) = &self.resource_type {
            and_clauses.push(format!("resource_type = '{}'", resource_type));
        }

        if let Some(ands) = &self._and {
            for and in ands {
                and_clauses.push(and.compile_sql());
            }
        }
        if let Some(ors) = &self._or {
            for or in ors {
                or_clauses.push(or.compile_sql());
            }
        }

        let mut where_clause = String::new();
        if !and_clauses.is_empty() {
            where_clause.push_str(&format!("({})", and_clauses.join(" AND ")));
        }
        if !or_clauses.is_empty() {
            if !where_clause.is_empty() {
                where_clause.push_str(" OR ");
            }
            where_clause.push_str(&format!("({})", or_clauses.join(" OR ")));
        }

        where_clause
    }
}

#[async_trait]
impl ChangeCrudOperations for SDKEngine {
    async fn create_change(&self, input: CreateChangeInput) -> Result<Change, SDKError> {
        let change_info = sqlx::query!(
            r#"
            INSERT INTO changes (owner_id, resource_id, operation, resource_type, diff_json)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
            input.owner_id,
            input.resource_id,
            input.operation.to_string(),
            input.resource_type.to_string(),
            input.diff_json,
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        Ok(Change {
            id: change_info.id,
            created_at: change_info.created_at,
            updated_at: change_info.updated_at,
            owner_id: change_info.owner_id,
            resource_id: change_info.resource_id,
            operation: ChangeOperation::from_str(change_info.operation.as_str()).unwrap(),
            resource_type: ChangeResourceType::from_str(change_info.resource_type.as_str()).unwrap(),
            diff_json: change_info.diff_json,
        })
    }

    async fn get_change(&self, id: Uuid) -> Result<Change, SDKError> {
        let change_info = sqlx::query!(
            r#"
            SELECT * FROM changes
            WHERE id = $1
            "#,
            id,
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        Ok(Change {
            id: change_info.id,
            created_at: change_info.created_at,
            updated_at: change_info.updated_at,
            owner_id: change_info.owner_id,
            resource_id: change_info.resource_id,
            operation: ChangeOperation::from_str(change_info.operation.as_str()).unwrap(),
            resource_type: ChangeResourceType::from_str(change_info.resource_type.as_str()).unwrap(),
            diff_json: change_info.diff_json,
        })
    }

    async fn get_changes(&self, input: GetChangesInput) -> Result<Vec<Change>, SDKError> {
        let mut query = "SELECT * FROM changes ".to_string();

        if let Some(filter) = input.filter {
            query.push_str(format!("WHERE {} ", filter.compile_sql()).as_str());
        }

        if let Some(sort_by) = input.sort_by {
            query.push_str(format!("ORDER BY {} ", sort_by).as_str());
        }

        if let Some(sort_order) = input.sort_order {
            query.push_str(format!("{} ", sort_order).as_str());
        }

        if let Some(limit) = input.limit {
            query.push_str(format!("LIMIT {} ", limit).as_str());
        }

        if let Some(offset) = input.offset {
            query.push_str(format!("OFFSET {} ", offset).as_str());
        }

        let changes_info = sqlx::query(query.as_str()).fetch_all(self.db_pool.as_ref()).await?;

        let changes = changes_info
            .into_iter()
            .map(|change_info| Change {
                id: change_info.get("id"),
                created_at: change_info.get("created_at"),
                updated_at: change_info.get("updated_at"),
                owner_id: change_info.get("owner_id"),
                resource_id: change_info.get("resource_id"),
                operation: ChangeOperation::from_str(change_info.get::<'_, String, _>("operation").as_str()).unwrap(),
                resource_type: ChangeResourceType::from_str(change_info.get::<'_, String, _>("resource_type").as_str())
                    .unwrap(),
                diff_json: change_info.get("diff_json"),
            })
            .collect();

        Ok(changes)
    }

    async fn update_change(&self, id: Uuid, input: UpdateChangeInput) -> Result<Change, SDKError> {
        let change_info = sqlx::query!(
            r#"
            UPDATE changes
            SET
                owner_id = COALESCE($1, owner_id),
                resource_id = COALESCE($2, resource_id),
                operation = COALESCE($3, operation),
                resource_type = COALESCE($4, resource_type),
                diff_json = COALESCE($5, diff_json)
            WHERE id = $6
            RETURNING *
            "#,
            input.owner_id,
            input.resource_id,
            input.operation.map(|op| op.to_string()),
            input.resource_type.map(|rt| rt.to_string()),
            input.diff_json,
            id,
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        Ok(Change {
            id: change_info.id,
            created_at: change_info.created_at,
            updated_at: change_info.updated_at,
            owner_id: change_info.owner_id,
            resource_id: change_info.resource_id,
            operation: ChangeOperation::from_str(change_info.operation.as_str()).unwrap(),
            resource_type: ChangeResourceType::from_str(change_info.resource_type.as_str()).unwrap(),
            diff_json: change_info.diff_json,
        })
    }

    async fn delete_change(&self, id: Uuid) -> Result<Change, SDKError> {
        let change_info = sqlx::query!(
            r#"
            DELETE FROM changes WHERE id = $1
            RETURNING *
            "#,
            id,
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        Ok(Change {
            id: change_info.id,
            created_at: change_info.created_at,
            updated_at: change_info.updated_at,
            owner_id: change_info.owner_id,
            resource_id: change_info.resource_id,
            operation: ChangeOperation::from_str(change_info.operation.as_str()).unwrap(),
            resource_type: ChangeResourceType::from_str(change_info.resource_type.as_str()).unwrap(),
            diff_json: change_info.diff_json,
        })
    }
}
