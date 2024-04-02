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
use crate::resources::chats::chat::{Chat, ChatStatus};

#[async_trait]
pub trait ChatCrudOperations {
    async fn create_chat(&self, input: CreateChatInput) -> Result<Chat, SDKError>;
    async fn get_chat(&self, id: Uuid) -> Result<Chat, SDKError>;
    async fn get_chats(&self, input: Option<GetChatsInput>) -> Result<Vec<Chat>, SDKError>;
    async fn update_chat(&self, id: Uuid, input: UpdateChatInput) -> Result<Chat, SDKError>;
    async fn delete_chat(&self, id: Uuid) -> Result<Chat, SDKError>;
}

#[derive(Default, Builder, Object, InputObject)]
#[builder(pattern = "owned")]
pub struct GetChatsInput {
    #[builder(setter(strip_option), default)]
    pub filter: Option<GetChatsWhere>,

    #[builder(setter(strip_option), default)]
    pub sort_by: Option<String>,
    #[builder(setter(strip_option), default)]
    pub sort_order: Option<SortOrder>,

    #[builder(setter(into, strip_option), default = "Some(100)")]
    pub limit: Option<i32>,
    #[builder(setter(into, strip_option), default = "Some(0)")]
    pub offset: Option<i32>,
}

#[derive(Default, Builder, Object, InputObject, Serialize)]
#[builder(pattern = "owned")]
pub struct CreateChatInput {
    #[graphql(skip)]
    pub owner_id: Uuid,

    pub resource_id: Uuid,
    pub resource_type: String,

    #[builder(setter(strip_option), default)]
    pub status: Option<ChatStatus>,
}

#[derive(Default, Builder, Object, InputObject, Serialize, Clone)]
#[builder(pattern = "owned")]
pub struct UpdateChatInput {
    #[builder(setter(strip_option), default)]
    pub status: Option<ChatStatus>,
}

#[derive(Default, Builder, Object, InputObject)]
#[builder(pattern = "owned")]
pub struct GetChatsWhere {
    #[builder(setter(strip_option), default)]
    pub ids: Option<Vec<Uuid>>,
    #[builder(setter(strip_option), default)]
    pub owner_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub resource_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub status: Option<ChatStatus>,

    #[oai(skip)]
    #[builder(setter(strip_option), default)]
    pub _and: Option<Vec<GetChatsWhere>>,

    #[oai(skip)]
    #[builder(setter(strip_option), default)]
    pub _or: Option<Vec<GetChatsWhere>>,
}

impl GetChatsWhere {
    pub fn compile_sql(&self) -> String {
        let mut conditions = Vec::new();

        if let Some(ids) = &self.ids {
            conditions.push(format!(
                "id = ANY(array[{}]::uuid[])",
                ids.iter()
                    .map(|x| format!("'{}'", x))
                    .collect::<Vec<String>>()
                    .join(",")
            ));
        }

        if let Some(owner_id) = &self.owner_id {
            conditions.push(format!("owner_id = '{}'", owner_id));
        }

        if let Some(resource_id) = &self.resource_id {
            conditions.push(format!("resource_id = '{}'", resource_id));
        }

        if let Some(status) = &self.status {
            conditions.push(format!("status = '{}'", status));
        }

        if let Some(ands) = &self._and {
            let and_conditions: Vec<String> = ands.iter().map(|and| and.compile_sql()).collect();
            conditions.push(format!("({})", and_conditions.join(" AND ")));
        }

        if let Some(ors) = &self._or {
            let or_conditions: Vec<String> = ors.iter().map(|or| or.compile_sql()).collect();
            conditions.push(format!("({})", or_conditions.join(" OR ")));
        }

        conditions.join(" AND ")
    }
}

#[async_trait]
impl ChatCrudOperations for SDKEngine {
    async fn create_chat(&self, input: CreateChatInput) -> Result<Chat, SDKError> {
        let mut tx = self.db_pool.begin().await?;

        let chat = sqlx::query!(
            r#"
            INSERT INTO chats (owner_id, resource_id, resource_type, status)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            input.owner_id,
            input.resource_id,
            input.resource_type,
            input.status.unwrap_or_default().to_string(),
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(Chat {
            id: chat.id,
            owner_id: chat.owner_id,
            resource_id: chat.resource_id,
            resource_type: chat.resource_type,
            status: chat
                .status
                .and_then(|a| ChatStatus::from_str(&a).ok())
                .unwrap_or_default(),
            created_at: chat.created_at,
            updated_at: chat.updated_at,
        })
    }

    async fn get_chat(&self, id: Uuid) -> Result<Chat, SDKError> {
        let chat = sqlx::query!(
            r#"
            SELECT * FROM chats WHERE id = $1
            "#,
            id,
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        Ok(Chat {
            id: chat.id,
            owner_id: chat.owner_id,
            resource_id: chat.resource_id,
            resource_type: chat.resource_type,
            status: chat
                .status
                .and_then(|a| ChatStatus::from_str(&a).ok())
                .unwrap_or_default(),
            created_at: chat.created_at,
            updated_at: chat.updated_at,
        })
    }

    async fn update_chat(&self, id: Uuid, input: UpdateChatInput) -> Result<Chat, SDKError> {
        let mut tx = self.db_pool.begin().await?;

        let chat = sqlx::query!(
            r#"
            UPDATE chats
            SET status = $1
            WHERE id = $2
            RETURNING *
            "#,
            input.status.unwrap_or_default().to_string(),
            id,
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(Chat {
            id: chat.id,
            owner_id: chat.owner_id,
            resource_id: chat.resource_id,
            resource_type: chat.resource_type,
            status: chat
                .status
                .and_then(|a| ChatStatus::from_str(&a).ok())
                .unwrap_or_default(),
            created_at: chat.created_at,
            updated_at: chat.updated_at,
        })
    }

    async fn delete_chat(&self, id: Uuid) -> Result<Chat, SDKError> {
        let chat = sqlx::query!(
            r#"
            DELETE FROM chats WHERE id = $1
            RETURNING *
            "#,
            id,
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        Ok(Chat {
            id: chat.id,
            owner_id: chat.owner_id,
            resource_id: chat.resource_id,
            resource_type: chat.resource_type,
            status: chat
                .status
                .and_then(|a| ChatStatus::from_str(&a).ok())
                .unwrap_or_default(),
            created_at: chat.created_at,
            updated_at: chat.updated_at,
        })
    }

    async fn get_chats(&self, input: Option<GetChatsInput>) -> Result<Vec<Chat>, SDKError> {
        let mut query = "SELECT * FROM chats".to_string();

        if let Some(input) = input {
            if let Some(filter) = input.filter {
                let where_clause = filter.compile_sql();
                if !where_clause.is_empty() {
                    query += " WHERE ";
                    query += &where_clause;
                }
            }

            if let Some(sort_by) = input.sort_by {
                query += " ORDER BY ";
                query += &sort_by;
            }

            if let Some(sort_order) = input.sort_order {
                query += " ";
                query += &sort_order.to_string();
            }

            if let Some(limit) = input.limit {
                query += " LIMIT ";
                query += &limit.to_string();
            }

            if let Some(offset) = input.offset {
                query += " OFFSET ";
                query += &offset.to_string();
            }
        }

        let chats_info = sqlx::query(query.as_str()).fetch_all(self.db_pool.as_ref()).await?;

        let chats = chats_info
            .iter()
            .map(|x| Chat {
                id: x.get("id"),
                owner_id: x.get("owner_id"),
                resource_id: x.get("resource_id"),
                resource_type: x.get("resource_type"),
                status: x
                    .get::<'_, Option<String>, _>("status")
                    .and_then(|a| ChatStatus::from_str(&a).ok())
                    .unwrap_or_default(),
                created_at: x.get("created_at"),
                updated_at: x.get("updated_at"),
            })
            .collect::<Vec<Chat>>();

        Ok(chats)
    }
}
