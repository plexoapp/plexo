use std::sync::Arc;

use async_trait::async_trait;

use super::chat::Chat;
use crate::{backend::loaders::SDKLoaders, errors::sdk::SDKError, resources::members::member::Member};

#[async_trait]
pub trait ChatRelations {
    async fn owner(&self, loaders: &SDKLoaders) -> Result<Member, SDKError>;
}

#[async_trait]
impl ChatRelations for Chat {
    async fn owner(&self, loaders: &SDKLoaders) -> Result<Member, SDKError> {
        loaders
            .member_loader
            .load_one(self.owner_id)
            .await
            .map_err(|e| SDKError::SQLXError(Arc::try_unwrap(e).unwrap()))
            .map(|member| member.unwrap())
    }
}
