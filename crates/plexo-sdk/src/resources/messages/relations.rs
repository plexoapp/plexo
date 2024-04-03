use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    backend::loaders::SDKLoaders,
    errors::sdk::SDKError,
    resources::{chats::chat::Chat, members::member::Member},
};

use super::message::Message;

#[async_trait]
pub trait MessageRelations {
    async fn owner(&self, loaders: &SDKLoaders) -> Result<Member, SDKError>;
    async fn chat(&self, loaders: &SDKLoaders) -> Result<Chat, SDKError>;
}

#[async_trait]
impl MessageRelations for Message {
    async fn owner(&self, loaders: &SDKLoaders) -> Result<Member, SDKError> {
        loaders
            .member_loader
            .load_one(self.owner_id)
            .await
            .map(|member| member.unwrap())
            .map_err(|e| SDKError::SQLXError(Arc::try_unwrap(e).unwrap()))
    }

    async fn chat(&self, loaders: &SDKLoaders) -> Result<Chat, SDKError> {
        loaders
            .chat_loader
            .load_one(self.chat_id)
            .await
            .map(|chat| chat.unwrap())
            .map_err(|e| SDKError::SQLXError(Arc::try_unwrap(e).unwrap()))
    }
}
