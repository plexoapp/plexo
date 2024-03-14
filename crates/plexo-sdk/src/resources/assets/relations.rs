use async_trait::async_trait;

use crate::{
    backend::loaders::SDKLoaders, errors::sdk::SDKError, resources::assets::asset::Asset,
    resources::members::member::Member, resources::projects::project::Project,
};

#[async_trait]
pub trait AssetRelations {
    async fn owner(&self, loaders: &SDKLoaders) -> Result<Member, SDKError>;
    async fn project(&self, loaders: &SDKLoaders) -> Result<Option<Project>, SDKError>;
}

#[async_trait]
impl AssetRelations for Asset {
    async fn owner(&self, loaders: &SDKLoaders) -> Result<Member, SDKError> {
        let data = loaders.member_loader.load_one(self.owner_id).await.unwrap().unwrap();

        Ok(data)
    }

    async fn project(&self, loaders: &SDKLoaders) -> Result<Option<Project>, SDKError> {
        let Some(project_id) = self.project_id else {
            return Ok(None);
        };

        let data = loaders.project_loader.load_one(project_id).await.unwrap().unwrap();

        Ok(Some(data))
    }
}
