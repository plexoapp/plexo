use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    backend::loaders::SDKLoaders,
    errors::sdk::SDKError,
    resources::{members::member::Member, tasks::task::Task},
};

use super::label::Label;

#[async_trait]
pub trait LabelRelations {
    async fn owner(&self, loaders: &SDKLoaders) -> Result<Member, SDKError>;
    async fn tasks(&self, loaders: &SDKLoaders) -> Result<Vec<Task>, SDKError>;
}

#[async_trait]
impl LabelRelations for Label {
    async fn owner(&self, loaders: &SDKLoaders) -> Result<Member, SDKError> {
        let data = loaders.member_loader.load_one(self.owner_id).await.unwrap().unwrap();

        Ok(data)
    }

    async fn tasks(&self, loaders: &SDKLoaders) -> Result<Vec<Task>, SDKError> {
        let ids: Vec<Uuid> = sqlx::query!(
            r#"
            SELECT task_id FROM labels_by_tasks
            WHERE label_id = $1
            "#,
            &self.id
        )
        .fetch_all(&*loaders.engine.db_pool)
        .await
        .unwrap()
        .into_iter()
        .map(|id| id.task_id)
        .collect();

        let tasks_map = loaders.task_loader.load_many(ids.clone()).await.unwrap();

        let tasks: &Vec<Task> = &ids.into_iter().map(|id| tasks_map.get(&id).unwrap().clone()).collect();

        Ok(tasks.clone())
    }
}
