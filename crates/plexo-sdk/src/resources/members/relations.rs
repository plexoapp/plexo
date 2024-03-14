use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    backend::loaders::SDKLoaders,
    errors::sdk::SDKError,
    resources::{projects::project::Project, tasks::task::Task, teams::team::Team},
};

use super::member::Member;

#[async_trait]
pub trait MemberRelations {
    async fn projects(&self, loaders: &SDKLoaders) -> Result<Vec<Project>, SDKError>;
    async fn tasks(&self, loaders: &SDKLoaders) -> Result<Vec<Task>, SDKError>;
    async fn teams(&self, loaders: &SDKLoaders) -> Result<Vec<Team>, SDKError>;
}

#[async_trait]
impl MemberRelations for Member {
    async fn projects(&self, loaders: &SDKLoaders) -> Result<Vec<Project>, SDKError> {
        let ids: Vec<Uuid> = sqlx::query!(
            r#"
            SELECT project_id FROM members_by_projects
            WHERE member_id = $1
            "#,
            &self.id
        )
        .fetch_all(&*loaders.engine.db_pool)
        .await
        .unwrap()
        .into_iter()
        .map(|id| id.project_id)
        .collect();

        let projects_map = loaders.project_loader.load_many(ids.clone()).await.unwrap();

        let projects: &Vec<Project> = &ids
            .into_iter()
            .map(|id| projects_map.get(&id).unwrap().clone())
            .collect();

        Ok(projects.clone())
    }

    async fn tasks(&self, loaders: &SDKLoaders) -> Result<Vec<Task>, SDKError> {
        let ids: Vec<Uuid> = sqlx::query!(
            r#"
            SELECT task_id FROM tasks_by_assignees
            WHERE assignee_id = $1
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

    async fn teams(&self, loaders: &SDKLoaders) -> Result<Vec<Team>, SDKError> {
        let ids: Vec<Uuid> = sqlx::query!(
            r#"
            SELECT team_id FROM members_by_teams
            WHERE member_id = $1
            "#,
            &self.id
        )
        .fetch_all(&*loaders.engine.db_pool)
        .await?
        .into_iter()
        .map(|id| id.team_id)
        .collect();

        let teams_map = loaders.team_loader.load_many(ids.clone()).await.unwrap();

        let teams: &Vec<Team> = &ids.into_iter().map(|id| teams_map.get(&id).unwrap().clone()).collect();

        Ok(teams.clone())
    }
}
