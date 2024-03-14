use std::str::FromStr;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    backend::loaders::SDKLoaders,
    errors::sdk::SDKError,
    resources::{
        assets::asset::{Asset, AssetKind},
        changes::change::{Change, ChangeOperation, ChangeResourceType},
        members::member::Member,
        tasks::task::{Task, TaskPriority, TaskStatus},
        teams::team::Team,
    },
};

use super::project::Project;

#[async_trait]
pub trait ProjectRelations {
    async fn owner(&self, loaders: &SDKLoaders) -> Result<Member, SDKError>;
    async fn lead(&self, loaders: &SDKLoaders) -> Result<Option<Member>, SDKError>;

    async fn tasks(&self, loaders: &SDKLoaders) -> Result<Vec<Task>, SDKError>;
    async fn members(&self, loaders: &SDKLoaders) -> Result<Vec<Member>, SDKError>;
    async fn assets(&self, loaders: &SDKLoaders) -> Result<Vec<Asset>, SDKError>;
    async fn teams(&self, loaders: &SDKLoaders) -> Result<Vec<Team>, SDKError>;

    async fn changes(&self, loaders: &SDKLoaders) -> Result<Vec<Change>, SDKError>;
}

#[async_trait]
impl ProjectRelations for Project {
    async fn owner(&self, loaders: &SDKLoaders) -> Result<Member, SDKError> {
        let data = loaders.member_loader.load_one(self.owner_id).await.unwrap().unwrap();

        Ok(data)
    }

    async fn lead(&self, loaders: &SDKLoaders) -> Result<Option<Member>, SDKError> {
        let Some(lead_id) = self.lead_id else {
            return Ok(None);
        };

        let data = loaders.member_loader.load_one(lead_id).await.unwrap().unwrap();

        Ok(Some(data))
    }

    async fn tasks(&self, loaders: &SDKLoaders) -> Result<Vec<Task>, SDKError> {
        let tasks = sqlx::query!(
            r#"
        SELECT * FROM tasks
        WHERE project_id = $1"#,
            &self.id
        )
        .fetch_all(&*loaders.engine.db_pool)
        .await
        .unwrap();

        Ok(tasks
            .iter()
            .map(|task| Task {
                id: task.id,
                created_at: task.created_at,
                updated_at: task.updated_at,
                title: task.title.clone(),
                description: task.description.clone(),
                status: task
                    .status
                    .clone()
                    .and_then(|a| TaskStatus::from_str(&a).ok())
                    .unwrap_or_default(),
                priority: task
                    .priority
                    .clone()
                    .and_then(|a| TaskPriority::from_str(&a).ok())
                    .unwrap_or_default(),
                due_date: task.due_date,
                project_id: task.project_id,
                lead_id: task.lead_id,
                owner_id: task.owner_id,
                count: task.count,
                parent_id: task.parent_id,
            })
            .collect())
    }

    async fn members(&self, loaders: &SDKLoaders) -> Result<Vec<Member>, SDKError> {
        let ids: Vec<Uuid> = sqlx::query!(
            r#"
            SELECT member_id FROM members_by_projects
            WHERE project_id = $1
            "#,
            &self.id
        )
        .fetch_all(&*loaders.engine.db_pool)
        .await
        .unwrap()
        .into_iter()
        .map(|id| id.member_id)
        .collect();

        let members_map = loaders.member_loader.load_many(ids.clone()).await.unwrap();

        let members: &Vec<Member> = &ids
            .into_iter()
            .map(|id| members_map.get(&id).unwrap().clone())
            .collect();

        Ok(members.clone())
    }

    async fn assets(&self, loaders: &SDKLoaders) -> Result<Vec<Asset>, SDKError> {
        let assets = sqlx::query!(
            r#"
        SELECT * FROM assets
        WHERE project_id = $1"#,
            &self.id
        )
        .fetch_all(&*loaders.engine.db_pool)
        .await
        .unwrap();

        Ok(assets
            .iter()
            .map(|asset| Asset {
                id: asset.id,
                created_at: asset.created_at,
                updated_at: asset.updated_at,
                name: asset.name.clone(),
                owner_id: asset.owner_id,
                kind: asset
                    .kind
                    .clone()
                    .and_then(|a| AssetKind::from_str(&a).ok())
                    .unwrap_or_default(),
                project_id: asset.project_id,
            })
            .collect())
    }

    async fn teams(&self, loaders: &SDKLoaders) -> Result<Vec<Team>, SDKError> {
        let ids: Vec<Uuid> = sqlx::query!(
            r#"
            SELECT team_id FROM teams_by_projects
            WHERE project_id = $1
            "#,
            &self.id
        )
        .fetch_all(&*loaders.engine.db_pool)
        .await
        .unwrap()
        .into_iter()
        .map(|id| id.team_id)
        .collect();

        let teams_map = loaders.team_loader.load_many(ids.clone()).await.unwrap();

        let teams: &Vec<Team> = &ids.into_iter().map(|id| teams_map.get(&id).unwrap().clone()).collect();

        Ok(teams.clone())
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
