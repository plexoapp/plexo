use crate::backend::engine::SDKEngine;
use crate::common::commons::SortOrder;
use crate::errors::sdk::SDKError;
use crate::resources::messages::message::{Message, MessageStatus};

use async_graphql::InputObject;
use async_trait::async_trait;
use derive_builder::Builder;
use poem_openapi::Object;
use serde::Serialize;
use sqlx::Row;
use std::str::FromStr;
use uuid::Uuid;

#[async_trait]
pub trait MessageCrudOperations {
    async fn create_message(&self, input: CreateMessageInput) -> Result<Message, SDKError>;
    async fn get_message(&self, id: Uuid) -> Result<Message, SDKError>;
    async fn get_messages(&self, input: GetMessagesInput) -> Result<Vec<Message>, SDKError>;
    async fn update_message(&self, id: Uuid, input: UpdateMessageInput) -> Result<Message, SDKError>;
    async fn delete_message(&self, id: Uuid) -> Result<Message, SDKError>;
}

#[derive(Default, Builder, Object, InputObject)]
#[builder(pattern = "owned")]
pub struct GetMessagesWhere {
    #[builder(setter(strip_option), default)]
    pub ids: Option<Vec<Uuid>>,
    #[builder(setter(strip_option), default)]
    pub chat_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub parent_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub resource_id: Option<Uuid>,

    #[oai(skip)]
    #[builder(setter(strip_option), default)]
    pub _and: Option<Vec<GetMessagesWhere>>,

    #[oai(skip)]
    #[builder(setter(strip_option), default)]
    pub _or: Option<Vec<GetMessagesWhere>>,
}

#[derive(Default, Builder, Object, InputObject)]
#[builder(pattern = "owned")]
pub struct GetMessagesInput {
    #[builder(setter(strip_option), default)]
    pub filter: Option<GetMessagesWhere>,

    #[builder(setter(strip_option), default)]
    pub sort_by: Option<String>,
    #[builder(setter(strip_option), default)]
    pub sort_order: Option<SortOrder>,

    #[builder(setter(into, strip_option), default = "Some(100)")]
    pub limit: Option<i32>,
    #[builder(setter(into, strip_option), default = "Some(0)")]
    pub offset: Option<i32>,
}

#[derive(Clone, Default, Builder, Object, InputObject, Serialize)]
#[builder(pattern = "owned")]
pub struct CreateMessageInput {
    pub owner_id: Uuid,
    pub chat_id: Uuid,
    // pub resource_id: Uuid,
    pub resource_type: String,
    pub content: String,

    #[builder(setter(strip_option), default)]
    pub status: Option<MessageStatus>,
    #[builder(setter(strip_option), default)]
    pub parent_id: Option<Uuid>,
}

#[derive(Default, Builder, Object, InputObject, Serialize, Clone)]
#[builder(pattern = "owned")]
pub struct UpdateMessageInput {
    #[builder(setter(strip_option), default)]
    pub content: Option<String>,
    #[builder(setter(strip_option), default)]
    pub status: Option<MessageStatus>,
}

impl GetMessagesWhere {
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

        if let Some(chat_id) = &self.chat_id {
            where_clause.push(format!("chat_id = '{}'", chat_id));
        }

        if let Some(parent_id) = &self.parent_id {
            where_clause.push(format!("parent_id = '{}'", parent_id));
        }

        if let Some(resource_id) = &self.resource_id {
            where_clause.push(format!("resource_id = '{}'", resource_id));
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
impl MessageCrudOperations for SDKEngine {
    async fn create_message(&self, input: CreateMessageInput) -> Result<Message, SDKError> {
        let mut tx = self.db_pool.begin().await?;

        let message = sqlx::query!(
            r#"
            INSERT INTO messages (owner_id, chat_id, resource_id, resource_type, parent_id, status)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
            input.owner_id,
            input.chat_id,
            input.chat_id,
            input.resource_type,
            input.parent_id,
            input.status.map(|s| s.to_string()),
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(Message {
            id: message.id,
            created_at: message.created_at,
            updated_at: message.updated_at,
            owner_id: message.owner_id,
            chat_id: message.chat_id,
            content: message.content,
            parent_id: message.parent_id,
            status: message
                .status
                .and_then(|a| MessageStatus::from_str(&a).ok())
                .unwrap_or_default(),
        })
    }

    async fn get_message(&self, id: Uuid) -> Result<Message, SDKError> {
        let message = sqlx::query!(
            r#"
            SELECT * FROM messages WHERE id = $1
            "#,
            id
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        Ok(Message {
            id: message.id,
            created_at: message.created_at,
            updated_at: message.updated_at,
            owner_id: message.owner_id,
            chat_id: message.chat_id,
            content: message.content,
            parent_id: message.parent_id,
            status: message
                .status
                .and_then(|a| MessageStatus::from_str(&a).ok())
                .unwrap_or_default(),
        })
    }

    async fn update_message(&self, id: Uuid, input: UpdateMessageInput) -> Result<Message, SDKError> {
        let mut tx = self.db_pool.begin().await?;

        let message = sqlx::query!(
            r#"
            UPDATE messages
            SET content = COALESCE($1, content), status = COALESCE($2, status)
            WHERE id = $3
            RETURNING *
            "#,
            input.content,
            input.status.map(|s| s.to_string()),
            id,
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(Message {
            id: message.id,
            created_at: message.created_at,
            updated_at: message.updated_at,
            owner_id: message.owner_id,
            chat_id: message.chat_id,
            content: message.content,
            parent_id: message.parent_id,
            status: message
                .status
                .and_then(|a| MessageStatus::from_str(&a).ok())
                .unwrap_or_default(),
        })
    }

    async fn delete_message(&self, id: Uuid) -> Result<Message, SDKError> {
        let message = sqlx::query!(
            r#"
            DELETE FROM messages WHERE id = $1
            RETURNING *
            "#,
            id
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        Ok(Message {
            id: message.id,
            created_at: message.created_at,
            updated_at: message.updated_at,
            owner_id: message.owner_id,
            chat_id: message.chat_id,
            content: message.content,
            parent_id: message.parent_id,
            status: message
                .status
                .and_then(|a| MessageStatus::from_str(&a).ok())
                .unwrap_or_default(),
        })
    }

    async fn get_messages(&self, input: GetMessagesInput) -> Result<Vec<Message>, SDKError> {
        let mut query = "SELECT * FROM messages ".to_string();

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

        let messages = sqlx::query(query.as_str()).fetch_all(self.db_pool.as_ref()).await?;

        let messages = messages
            .iter()
            .map(|x| Message {
                id: x.get("id"),
                created_at: x.get("created_at"),
                updated_at: x.get("updated_at"),
                owner_id: x.get("owner_id"),
                chat_id: x.get("chat_id"),
                content: x.get("content"),
                parent_id: x.get("parent_id"),
                status: x
                    .get::<'_, Option<String>, _>("status")
                    .and_then(|a| MessageStatus::from_str(&a).ok())
                    .unwrap_or_default(),
            })
            .collect();

        Ok(messages)
    }
}
