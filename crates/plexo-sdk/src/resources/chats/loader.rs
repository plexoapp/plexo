use super::chat::{Chat, ChatStatus};
use crate::backend::engine::SDKEngine;
use async_graphql::dataloader::Loader;
use std::{collections::HashMap, str::FromStr, sync::Arc};
use uuid::Uuid;

pub struct ChatLoader(Arc<SDKEngine>);

impl ChatLoader {
    pub fn new(e: Arc<SDKEngine>) -> Self {
        Self(e)
    }
}

impl Loader<Uuid> for ChatLoader {
    type Value = Chat;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &'_ [Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let messages = sqlx::query!(
            r#"
            SELECT * FROM chats WHERE id = ANY($1)
            "#,
            &keys
        )
        .fetch_all(&*self.0.db_pool)
        .await?
        .iter()
        .map(|message| {
            (
                message.id,
                Chat {
                    id: message.id,
                    owner_id: message.owner_id,
                    resource_id: message.resource_id,
                    resource_type: message.resource_type.to_string(),
                    status: message
                        .status
                        .as_ref()
                        .and_then(|a| ChatStatus::from_str(a).ok())
                        .unwrap_or_default(),
                    created_at: message.created_at,
                    updated_at: message.updated_at,
                },
            )
        })
        .collect();

        Ok(messages)
    }
}
