use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    backend::loaders::SDKLoaders, errors::sdk::SDKError, resources::members::member::Member,
    resources::projects::project::Project,
};

use super::team::Team;

#[async_trait]
pub trait TeamRelations {
    async fn owner(&self, loaders: &SDKLoaders) -> Result<Member, SDKError>;
    async fn projects(&self, loaders: &SDKLoaders) -> Result<Vec<Project>, SDKError>;
    async fn members(&self, loaders: &SDKLoaders) -> Result<Vec<Member>, SDKError>;
}

#[async_trait]
impl TeamRelations for Team {
    async fn owner(&self, loaders: &SDKLoaders) -> Result<Member, SDKError> {
        let data = loaders.member_loader.load_one(self.owner_id).await.unwrap().unwrap();

        Ok(data)
    }

    async fn projects(&self, loaders: &SDKLoaders) -> Result<Vec<Project>, SDKError> {
        let ids: Vec<Uuid> = sqlx::query!(
            r#"
            SELECT project_id FROM teams_by_projects
            WHERE team_id = $1
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

    async fn members(&self, loaders: &SDKLoaders) -> Result<Vec<Member>, SDKError> {
        let ids: Vec<Uuid> = sqlx::query!(
            r#"
            SELECT member_id FROM members_by_teams
            WHERE team_id = $1
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
}
