use std::str::FromStr;

use async_graphql::InputObject;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use poem_openapi::Object;
use serde::Serialize;
use sqlx::Row;
use uuid::Uuid;

use crate::{
    backend::engine::SDKEngine,
    common::commons::{SortOrder, UpdateListInput},
    errors::sdk::SDKError,
};

use super::project::{Project, ProjectStatus, ProjectVisibility};

#[async_trait]
pub trait ProjectCrudOperations {
    async fn create_project(&self, input: CreateProjectInput) -> Result<Project, SDKError>;
    async fn get_project(&self, id: Uuid) -> Result<Project, SDKError>;
    async fn get_projects(&self, input: GetProjectsInput) -> Result<Vec<Project>, SDKError>;
    async fn update_project(&self, id: Uuid, input: UpdateProjectInput) -> Result<Project, SDKError>;
    async fn delete_project(&self, id: Uuid) -> Result<Project, SDKError>;
}

#[derive(Clone, Default, Builder, Object, InputObject, Serialize)]
#[builder(pattern = "owned")]
pub struct CreateProjectInput {
    pub name: String,

    #[graphql(skip)]
    pub owner_id: Uuid,

    #[builder(setter(strip_option), default)]
    pub status: Option<ProjectStatus>,
    #[builder(setter(strip_option), default)]
    pub visibility: Option<ProjectVisibility>,

    #[builder(setter(strip_option), default)]
    pub prefix: Option<String>,
    #[builder(setter(strip_option), default)]
    pub description: Option<String>,
    #[builder(setter(strip_option), default)]
    pub lead_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub start_date: Option<DateTime<Utc>>,
    #[builder(setter(strip_option), default)]
    pub due_date: Option<DateTime<Utc>>,

    #[builder(setter(strip_option), default)]
    pub members: Option<Vec<Uuid>>,
    #[builder(setter(strip_option), default)]
    pub teams: Option<Vec<Uuid>>,
}

#[derive(Clone, Default, Builder, Object, InputObject, Serialize)]
#[builder(pattern = "owned")]
pub struct UpdateProjectInput {
    #[builder(setter(strip_option), default)]
    pub name: Option<String>,
    #[builder(setter(strip_option), default)]
    pub prefix: Option<String>,
    #[builder(setter(strip_option), default)]
    pub description: Option<String>,
    #[builder(setter(strip_option), default)]
    pub lead_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub start_date: Option<DateTime<Utc>>,
    #[builder(setter(strip_option), default)]
    pub due_date: Option<DateTime<Utc>>,

    #[builder(setter(strip_option), default)]
    pub status: Option<ProjectStatus>,
    #[builder(setter(strip_option), default)]
    pub visibility: Option<ProjectVisibility>,

    #[builder(setter(strip_option), default)]
    pub members: Option<UpdateListInput>,
    #[builder(setter(strip_option), default)]
    pub teams: Option<UpdateListInput>,
}

#[derive(Default, Builder, Object, InputObject)]
#[builder(pattern = "owned")]
pub struct GetProjectsInput {
    #[builder(setter(strip_option), default)]
    pub filter: Option<GetProjectsWhere>,

    #[builder(setter(strip_option), default)]
    pub sort_by: Option<String>,
    #[builder(setter(strip_option), default)]
    pub sort_order: Option<SortOrder>,

    #[builder(setter(into, strip_option), default = "Some(100)")]
    pub limit: Option<i32>,
    #[builder(setter(into, strip_option), default = "Some(0)")]
    pub offset: Option<i32>,
}

#[derive(Default, Builder, Object, InputObject)]
#[builder(pattern = "owned")]
pub struct GetProjectsWhere {
    #[builder(setter(strip_option), default)]
    pub ids: Option<Vec<Uuid>>,
    #[builder(setter(strip_option), default)]
    pub name: Option<String>,
    #[builder(setter(strip_option), default)]
    pub prefix: Option<String>,
    #[builder(setter(strip_option), default)]
    pub owner_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub description: Option<String>,
    #[builder(setter(strip_option), default)]
    pub lead_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub start_date: Option<DateTime<Utc>>,
    #[builder(setter(strip_option), default)]
    pub due_date: Option<DateTime<Utc>>,

    #[oai(skip)]
    #[builder(setter(strip_option), default)]
    pub _and: Option<Vec<GetProjectsWhere>>,

    #[oai(skip)]
    #[builder(setter(strip_option), default)]
    pub _or: Option<Vec<GetProjectsWhere>>,
}

impl GetProjectsWhere {
    pub fn compile_sql(&self) -> String {
        let mut where_clause = Vec::new();

        if let Some(ids) = &self.ids {
            where_clause.push(format!(
                "id = ANY(array[{}]::uuid[])",
                ids.iter()
                    .map(|x| format!("'{}'", x))
                    .collect::<Vec<String>>()
                    .join(",")
            ));
        }

        if let Some(name) = &self.name {
            where_clause.push(format!("name = '{}'", name));
        }
        if let Some(prefix) = &self.prefix {
            where_clause.push(format!("prefix = '{}'", prefix));
        }
        if let Some(owner_id) = &self.owner_id {
            where_clause.push(format!("owner_id = '{}'", owner_id));
        }
        if let Some(description) = &self.description {
            where_clause.push(format!("description = '{}'", description));
        }
        if let Some(lead_id) = &self.lead_id {
            where_clause.push(format!("lead_id = '{}'", lead_id));
        }
        if let Some(start_date) = &self.start_date {
            where_clause.push(format!("start_date = '{}'", start_date));
        }
        if let Some(due_date) = &self.due_date {
            where_clause.push(format!("due_date = '{}'", due_date));
        }
        if let Some(_and) = &self._and {
            where_clause.push(format!(
                "({})",
                _and.iter()
                    .map(|x| x.compile_sql())
                    .collect::<Vec<String>>()
                    .join(" AND ")
            ));
        }
        if let Some(_or) = &self._or {
            where_clause.push(format!(
                "({})",
                _or.iter()
                    .map(|x| x.compile_sql())
                    .collect::<Vec<String>>()
                    .join(" OR ")
            ));
        }

        where_clause.join(" AND ")
    }
}

#[async_trait]
impl ProjectCrudOperations for SDKEngine {
    async fn create_project(&self, input: CreateProjectInput) -> Result<Project, SDKError> {
        let mut tx = self.db_pool.as_ref().begin().await?;

        let project = sqlx::query!(
            r#"
            INSERT INTO projects (name, description, owner_id, status)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            input.name,
            input.description,
            input.owner_id,
            input.status.unwrap_or_default().to_string(),
        )
        .fetch_one(&mut *tx)
        .await?;

        if let Some(members) = input.members {
            for member in members {
                sqlx::query!(
                    r#"
                        INSERT INTO members_by_projects (member_id, project_id)
                        VALUES ($1, $2)
                        "#,
                    member,
                    project.id,
                )
                .execute(&mut *tx)
                .await
                .unwrap();
            }
        }

        if let Some(teams) = input.teams {
            for team in teams {
                sqlx::query!(
                    r#"
                        INSERT INTO teams_by_projects (team_id, project_id)
                        VALUES ($1, $2)
                        "#,
                    team,
                    project.id,
                )
                .execute(&mut *tx)
                .await
                .unwrap();
            }
        }

        tx.commit().await?;

        Ok(Project {
            id: project.id,
            created_at: project.created_at,
            updated_at: project.updated_at,
            name: project.name,
            prefix: project.prefix,
            owner_id: project.owner_id,
            description: project.description,
            lead_id: project.lead_id,
            start_date: project.start_date,
            due_date: project.due_date,
            status: project
                .status
                .and_then(|a| ProjectStatus::from_str(&a).ok())
                .unwrap_or_default(),
            visibility: project
                .visibility
                .and_then(|a| ProjectVisibility::from_str(&a).ok())
                .unwrap_or_default(),
        })
    }

    async fn get_project(&self, id: Uuid) -> Result<Project, SDKError> {
        let project_info = sqlx::query!(
            r#"
            SELECT * FROM projects WHERE id = $1
            "#,
            id,
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        Ok(Project {
            id: project_info.id,
            created_at: project_info.created_at,
            updated_at: project_info.updated_at,
            name: project_info.name,
            prefix: project_info.prefix,
            owner_id: project_info.owner_id,
            description: project_info.description,
            lead_id: project_info.lead_id,
            start_date: project_info.start_date,
            due_date: project_info.due_date,
            status: project_info
                .status
                .and_then(|a| ProjectStatus::from_str(&a).ok())
                .unwrap_or_default(),
            visibility: project_info
                .visibility
                .and_then(|a| ProjectVisibility::from_str(&a).ok())
                .unwrap_or_default(),
        })
    }

    async fn update_project(&self, id: Uuid, input: UpdateProjectInput) -> Result<Project, SDKError> {
        let mut tx = self.db_pool.as_ref().begin().await?;

        let project_final_info = sqlx::query!(
            r#"
            UPDATE projects
            SET
                name = COALESCE($1, name),
                description = COALESCE($2, description),
                prefix = COALESCE($3, prefix),
                lead_id = COALESCE($4, lead_id),
                start_date = COALESCE($5, start_date),
                due_date = COALESCE($6, due_date),
                status = COALESCE($7, status),
                visibility = COALESCE($8, visibility)
            WHERE id = $9
            RETURNING *
            "#,
            input.name,
            input.description,
            input.prefix,
            input.lead_id,
            input.start_date,
            input.due_date,
            input.status.map(|a| a.to_string()),
            input.visibility.map(|a| a.to_string()),
            id,
        )
        .fetch_one(&mut *tx)
        .await?;

        if let Some(members) = input.members {
            for member in members.add {
                sqlx::query!(
                    r#"
                        INSERT INTO members_by_projects (member_id, project_id)
                        VALUES ($1, $2)
                        "#,
                    member,
                    id,
                )
                .execute(&mut *tx)
                .await
                .unwrap();
            }

            for member in members.remove {
                sqlx::query!(
                    r#"
                        DELETE FROM members_by_projects
                        WHERE member_id = $1 AND project_id = $2
                        "#,
                    member,
                    id,
                )
                .execute(&mut *tx)
                .await
                .unwrap();
            }
        }

        if let Some(teams) = input.teams {
            for team in teams.add {
                sqlx::query!(
                    r#"
                        INSERT INTO teams_by_projects (team_id, project_id)
                        VALUES ($1, $2)
                        "#,
                    team,
                    id,
                )
                .execute(&mut *tx)
                .await
                .unwrap();
            }

            for team in teams.remove {
                sqlx::query!(
                    r#"
                        DELETE FROM teams_by_projects
                        WHERE team_id = $1 AND project_id = $2
                        "#,
                    team,
                    id,
                )
                .execute(&mut *tx)
                .await
                .unwrap();
            }
        }

        tx.commit().await?;

        Ok(Project {
            id: project_final_info.id,
            created_at: project_final_info.created_at,
            updated_at: project_final_info.updated_at,
            name: project_final_info.name,
            prefix: project_final_info.prefix,
            owner_id: project_final_info.owner_id,
            description: project_final_info.description,
            lead_id: project_final_info.lead_id,
            start_date: project_final_info.start_date,
            due_date: project_final_info.due_date,
            status: project_final_info
                .status
                .and_then(|a| ProjectStatus::from_str(&a).ok())
                .unwrap_or_default(),
            visibility: project_final_info
                .visibility
                .and_then(|a| ProjectVisibility::from_str(&a).ok())
                .unwrap_or_default(),
        })
    }

    async fn delete_project(&self, id: Uuid) -> Result<Project, SDKError> {
        let project_info = sqlx::query!(
            r#"
            DELETE FROM projects WHERE id = $1
            RETURNING *
            "#,
            id,
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        Ok(Project {
            id: project_info.id,
            created_at: project_info.created_at,
            updated_at: project_info.updated_at,
            name: project_info.name,
            prefix: project_info.prefix,
            owner_id: project_info.owner_id,
            description: project_info.description,
            lead_id: project_info.lead_id,
            start_date: project_info.start_date,
            due_date: project_info.due_date,
            status: project_info
                .status
                .and_then(|a| ProjectStatus::from_str(&a).ok())
                .unwrap_or_default(),
            visibility: project_info
                .visibility
                .and_then(|a| ProjectVisibility::from_str(&a).ok())
                .unwrap_or_default(),
        })
    }

    async fn get_projects(&self, input: GetProjectsInput) -> Result<Vec<Project>, SDKError> {
        let mut query = "SELECT * FROM projects ".to_string();

        if let Some(filter) = input.filter {
            query.push_str(format!("WHERE {} ", filter.compile_sql()).as_str());
        }

        if let Some(sort_by) = input.sort_by {
            query.push_str(format!("ORDER BY {} ", sort_by).as_str());
        }

        if let Some(sort_order) = input.sort_order {
            query.push_str(format!("{} ", sort_order).as_str());
        }

        if let Some(limit) = input.limit {
            query.push_str(format!("LIMIT {} ", limit).as_str());
        }

        if let Some(offset) = input.offset {
            query.push_str(format!("OFFSET {} ", offset).as_str());
        }

        let projects_info = sqlx::query(query.as_str()).fetch_all(self.db_pool.as_ref()).await?;

        let projects = projects_info
            .iter()
            .map(|x| Project {
                id: x.get("id"),
                created_at: x.get("created_at"),
                updated_at: x.get("updated_at"),
                name: x.get("name"),
                prefix: x.get("prefix"),
                owner_id: x.get("owner_id"),
                description: x.get("description"),
                lead_id: x.get("lead_id"),
                start_date: x.get("start_date"),
                due_date: x.get("due_date"),
                status: x
                    .get::<'_, Option<String>, _>("status")
                    .and_then(|a| ProjectStatus::from_str(&a).ok())
                    .unwrap_or_default(),
                visibility: x
                    .get::<'_, Option<String>, _>("visibility")
                    .and_then(|a| ProjectVisibility::from_str(&a).ok())
                    .unwrap_or_default(),
            })
            .collect::<Vec<Project>>();

        Ok(projects)
    }
}
