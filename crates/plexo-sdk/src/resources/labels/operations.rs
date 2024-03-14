use async_graphql::InputObject;
use async_trait::async_trait;
use derive_builder::Builder;
use poem_openapi::Object;
use serde::Serialize;
use sqlx::Row;
use uuid::Uuid;

use crate::{backend::engine::SDKEngine, common::commons::SortOrder, errors::sdk::SDKError};

use super::label::Label;

#[async_trait]
pub trait LabelCrudOperations {
    async fn create_label(&self, input: CreateLabelInput) -> Result<Label, SDKError>;
    async fn get_label(&self, id: Uuid) -> Result<Label, SDKError>;
    async fn get_labels(&self, input: GetLabelsInput) -> Result<Vec<Label>, SDKError>;
    async fn update_label(&self, id: Uuid, input: UpdateLabelInput) -> Result<Label, SDKError>;
    async fn delete_label(&self, id: Uuid) -> Result<Label, SDKError>;
}

#[derive(Clone, Default, Builder, Object, InputObject, Serialize)]
#[builder(pattern = "owned")]
pub struct CreateLabelInput {
    pub name: String,

    #[graphql(skip)]
    pub owner_id: Uuid,

    #[builder(setter(strip_option), default)]
    pub description: Option<String>,
    #[builder(setter(strip_option), default)]
    pub color: Option<String>,
}

#[derive(Clone, Default, Builder, Object, InputObject, Serialize)]
#[builder(pattern = "owned")]
pub struct UpdateLabelInput {
    #[builder(setter(strip_option), default)]
    pub name: Option<String>,
    #[builder(setter(strip_option), default)]
    pub description: Option<String>,
    #[builder(setter(strip_option), default)]
    pub color: Option<String>,
}

#[derive(Default, Builder, Object, InputObject)]
#[builder(pattern = "owned")]
pub struct GetLabelsInput {
    #[builder(setter(strip_option), default)]
    pub filter: Option<GetLabelsWhere>,

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
pub struct GetLabelsWhere {
    #[builder(setter(strip_option), default)]
    pub ids: Option<Vec<Uuid>>,
    #[builder(setter(strip_option), default)]
    pub name: Option<String>,
    #[builder(setter(strip_option), default)]
    pub description: Option<String>,
    #[builder(setter(strip_option), default)]
    pub color: Option<String>,

    #[oai(skip)]
    #[builder(setter(strip_option), default)]
    pub _and: Option<Vec<GetLabelsWhere>>,
    #[oai(skip)]
    #[builder(setter(strip_option), default)]
    pub _or: Option<Vec<GetLabelsWhere>>,
}

impl GetLabelsWhere {
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

        if let Some(name) = &self.name {
            and_clauses.push(format!("name = '{}'", name));
        }
        if let Some(description) = &self.description {
            and_clauses.push(format!("description = '{}'", description));
        }
        if let Some(color) = &self.color {
            and_clauses.push(format!("color = '{}'", color));
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
impl LabelCrudOperations for SDKEngine {
    async fn create_label(&self, input: CreateLabelInput) -> Result<Label, SDKError> {
        let label_info = sqlx::query!(
            r#"
            INSERT INTO labels (name, description, color, owner_id)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            input.name,
            input.description,
            input.color,
            input.owner_id,
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        Ok(Label {
            id: label_info.id,
            created_at: label_info.created_at,
            updated_at: label_info.updated_at,
            name: label_info.name,
            owner_id: label_info.owner_id,
            description: label_info.description,
            color: label_info.color,
        })
    }

    async fn get_label(&self, id: Uuid) -> Result<Label, SDKError> {
        let label_info = sqlx::query!(
            r#"
            SELECT * FROM labels
            WHERE id = $1
            "#,
            id,
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        Ok(Label {
            id: label_info.id,
            created_at: label_info.created_at,
            updated_at: label_info.updated_at,
            name: label_info.name,
            owner_id: label_info.owner_id,
            description: label_info.description,
            color: label_info.color,
        })
    }

    async fn get_labels(&self, input: GetLabelsInput) -> Result<Vec<Label>, SDKError> {
        let mut query = "SELECT * FROM labels ".to_string();

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

        let labels_info = sqlx::query(query.as_str()).fetch_all(self.db_pool.as_ref()).await?;

        let labels = labels_info
            .into_iter()
            .map(|label_info| Label {
                id: label_info.get("id"),
                created_at: label_info.get("created_at"),
                updated_at: label_info.get("updated_at"),
                name: label_info.get("name"),
                owner_id: label_info.get("owner_id"),
                description: label_info.get("description"),
                color: label_info.get("color"),
            })
            .collect();

        Ok(labels)
    }

    async fn update_label(&self, id: Uuid, input: UpdateLabelInput) -> Result<Label, SDKError> {
        let label_info = sqlx::query!(
            r#"
            UPDATE labels
            SET
                name = COALESCE($1, name),
                description = COALESCE($2, description),
                color = COALESCE($3, color)
            WHERE id = $4
            RETURNING *
            "#,
            input.name,
            input.description,
            input.color,
            id,
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        Ok(Label {
            id: label_info.id,
            created_at: label_info.created_at,
            updated_at: label_info.updated_at,
            name: label_info.name,
            owner_id: label_info.owner_id,
            description: label_info.description,
            color: label_info.color,
        })
    }

    async fn delete_label(&self, id: Uuid) -> Result<Label, SDKError> {
        let label_info = sqlx::query!(
            r#"
            DELETE FROM labels WHERE id = $1
            RETURNING *
            "#,
            id,
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        Ok(Label {
            id: label_info.id,
            created_at: label_info.created_at,
            updated_at: label_info.updated_at,
            name: label_info.name,
            owner_id: label_info.owner_id,
            description: label_info.description,
            color: label_info.color,
        })
    }
}
