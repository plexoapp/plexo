use async_trait::async_trait;

use crate::{backend::loaders::SDKLoaders, errors::sdk::SDKError, resources::members::member::Member};

use super::change::Change;

#[async_trait]
pub trait ChangeRelations {
    async fn owner(&self, loaders: &SDKLoaders) -> Result<Member, SDKError>;
    // async fn tasks(&self) -> Result<Vec<Task>, SDKError>;
    // async fn lead(&self) -> Result<Member, SDKError>;
    // async fn assets(&self) -> Result<Vec<Asset>, SDKError>;
}

#[async_trait]
impl ChangeRelations for Change {
    async fn owner(&self, loaders: &SDKLoaders) -> Result<Member, SDKError> {
        let data = loaders.member_loader.load_one(self.owner_id).await.unwrap().unwrap();

        Ok(data)
    }
}
