use std::{collections::HashMap, str::FromStr, sync::Arc};

use async_graphql::dataloader::Loader;

use uuid::Uuid;

use crate::backend::engine::SDKEngine;

use super::project::{Project, ProjectStatus, ProjectVisibility};

pub struct ProjectLoader(Arc<SDKEngine>);

impl ProjectLoader {
    pub fn new(e: Arc<SDKEngine>) -> Self {
        Self(e)
    }
}

impl Loader<Uuid> for ProjectLoader {
    type Value = Project;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &'_ [Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let projects = sqlx::query!(
            r#"
            SELECT * FROM projects WHERE id  = ANY($1)
            "#,
            &keys
        )
        .fetch_all(&*self.0.db_pool)
        .await
        .unwrap();

        //iterate to get the hashmap
        let projects_map: HashMap<Uuid, Project> = projects
            .iter()
            .map(|project| {
                (
                    project.id,
                    Project {
                        id: project.id,
                        created_at: project.created_at,
                        updated_at: project.updated_at,
                        name: project.name.clone(),
                        prefix: project.prefix.clone(),
                        owner_id: project.owner_id,
                        description: project.description.clone(),
                        lead_id: project.lead_id,
                        start_date: project.start_date,
                        due_date: project.due_date,
                        status: project
                            .status
                            .clone()
                            .and_then(|a| ProjectStatus::from_str(&a).ok())
                            .unwrap_or_default(),
                        visibility: project
                            .visibility
                            .clone()
                            .and_then(|a| ProjectVisibility::from_str(&a).ok())
                            .unwrap_or_default(),
                    },
                )
            })
            .collect();

        //println!("{:?}", projects);
        Ok(projects_map)
    }
}
