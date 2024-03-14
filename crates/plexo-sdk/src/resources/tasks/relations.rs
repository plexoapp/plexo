use std::str::FromStr;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    backend::loaders::SDKLoaders,
    errors::sdk::SDKError,
    resources::{
        changes::change::{Change, ChangeOperation, ChangeResourceType},
        labels::label::Label,
        members::member::Member,
        projects::project::Project,
    },
};

use super::task::Task;

#[async_trait]
pub trait TaskRelations {
    async fn owner(&self, loaders: &SDKLoaders) -> Result<Member, SDKError>;
    async fn project(&self, loaders: &SDKLoaders) -> Result<Option<Project>, SDKError>;
    async fn lead(&self, loaders: &SDKLoaders) -> Result<Option<Member>, SDKError>;
    async fn parent(&self, loaders: &SDKLoaders) -> Result<Option<Task>, SDKError>;

    async fn assignees(&self, loaders: &SDKLoaders) -> Result<Vec<Member>, SDKError>;
    async fn labels(&self, loaders: &SDKLoaders) -> Result<Vec<Label>, SDKError>;
    async fn subtasks(&self, loaders: &SDKLoaders) -> Result<Vec<Task>, SDKError>;
    async fn changes(&self, loaders: &SDKLoaders) -> Result<Vec<Change>, SDKError>;
}

#[async_trait]
impl TaskRelations for Task {
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

    async fn lead(&self, loaders: &SDKLoaders) -> Result<Option<Member>, SDKError> {
        let Some(lead_id) = self.lead_id else {
            return Ok(None);
        };

        let data = loaders.member_loader.load_one(lead_id).await.unwrap().unwrap();

        Ok(Some(data))
    }

    async fn parent(&self, loaders: &SDKLoaders) -> Result<Option<Task>, SDKError> {
        let Some(parent_id) = self.parent_id else {
            return Ok(None);
        };

        let data = loaders.task_loader.load_one(parent_id).await.unwrap().unwrap();

        Ok(Some(data))
    }

    async fn assignees(&self, loaders: &SDKLoaders) -> Result<Vec<Member>, SDKError> {
        let ids: Vec<Uuid> = sqlx::query!(
            r#"
            SELECT assignee_id FROM tasks_by_assignees
            WHERE task_id = $1
            "#,
            &self.id
        )
        .fetch_all(&*loaders.engine.db_pool)
        .await
        .unwrap()
        .into_iter()
        .map(|id| id.assignee_id)
        .collect();

        let members_map = loaders.member_loader.load_many(ids.clone()).await.unwrap();

        let members: &Vec<Member> = &ids
            .into_iter()
            .map(|id| members_map.get(&id).unwrap().clone())
            .collect();

        Ok(members.clone())
    }

    async fn labels(&self, loaders: &SDKLoaders) -> Result<Vec<Label>, SDKError> {
        let ids: Vec<Uuid> = sqlx::query!(
            r#"
            SELECT label_id FROM labels_by_tasks
            WHERE task_id = $1
            "#,
            &self.id
        )
        .fetch_all(&*loaders.engine.db_pool)
        .await
        .unwrap()
        .into_iter()
        .map(|id| id.label_id)
        .collect();

        let labels_map = loaders.label_loader.load_many(ids.clone()).await.unwrap();

        let labels: &Vec<Label> = &ids.into_iter().map(|id| labels_map.get(&id).unwrap().clone()).collect();

        Ok(labels.clone())
    }

    async fn subtasks(&self, loaders: &SDKLoaders) -> Result<Vec<Task>, SDKError> {
        let ids: Vec<Uuid> = sqlx::query!(
            r#"
            SELECT id FROM tasks
            WHERE parent_id = $1
            "#,
            &self.id
        )
        .fetch_all(&*loaders.engine.db_pool)
        .await
        .unwrap()
        .into_iter()
        .map(|t| t.id)
        .collect();

        Ok(loaders
            .task_loader
            .load_many(ids)
            .await
            .unwrap()
            .values()
            .map(|x| x.to_owned())
            .collect())
    }

    async fn changes(&self, loaders: &SDKLoaders) -> Result<Vec<Change>, SDKError> {
        let changes = sqlx::query!(
            r#"
        SELECT * FROM changes
        WHERE resource_id = $1"#,
            &self.id
        )
        .fetch_all(&*loaders.engine.db_pool)
        .await
        .unwrap();

        Ok(changes
            .iter()
            .map(|change| Change {
                id: change.id,
                created_at: change.created_at,
                updated_at: change.updated_at,
                owner_id: change.owner_id,
                resource_id: change.resource_id,
                operation: ChangeOperation::from_str(change.operation.as_str()).unwrap(),
                resource_type: ChangeResourceType::from_str(change.resource_type.as_str()).unwrap(),
                diff_json: change.diff_json.clone(),
            })
            .collect())
    }
}
