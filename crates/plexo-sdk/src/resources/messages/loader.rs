use super::message::{Message, MessageStatus};
use crate::backend::engine::SDKEngine;
use async_graphql::dataloader::Loader;
use std::{collections::HashMap, str::FromStr, sync::Arc};
use uuid::Uuid;

pub struct MessageLoader(Arc<SDKEngine>);

impl MessageLoader {
    pub fn new(e: Arc<SDKEngine>) -> Self {
        Self(e)
    }
}

impl Loader<Uuid> for MessageLoader {
    type Value = Message;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &'_ [Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let messages = sqlx::query!(
            r#"
            SELECT * FROM messages WHERE id = ANY($1)
            "#,
            &keys
        )
        .fetch_all(&*self.0.db_pool)
        .await?
        .iter()
        .map(|message| {
            (
                message.id,
                Message {
                    id: message.id,
                    created_at: message.created_at,
                    updated_at: message.updated_at,
                    owner_id: message.owner_id,
                    chat_id: message.chat_id,
                    content: message.content.clone(),
                    status: message
                        .status
                        .as_ref()
                        .and_then(|a| MessageStatus::from_str(a).ok())
                        .unwrap_or_default(),
                    parent_id: message.parent_id,
                },
            )
        })
        .collect();

        Ok(messages)
    }
}
