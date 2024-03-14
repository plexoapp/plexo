use std::{collections::HashMap, str::FromStr, sync::Arc};

use async_graphql::dataloader::Loader;

use uuid::Uuid;

use crate::backend::engine::SDKEngine;

use super::task::{Task, TaskPriority, TaskStatus};

// #[derive(Clone)]
pub struct TaskLoader(Arc<SDKEngine>);

impl TaskLoader {
    pub fn new(e: Arc<SDKEngine>) -> Self {
        Self(e)
    }
}

impl Loader<Uuid> for TaskLoader {
    type Value = Task;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &'_ [Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let tasks = sqlx::query!(
            r#"
            SELECT * FROM tasks WHERE id  = ANY($1)
            "#,
            &keys
        )
        .fetch_all(&*self.0.db_pool)
        .await
        .unwrap();

        //iterate to get the hashmap
        let tasks_map: HashMap<Uuid, Task> = tasks
            .iter()
            .map(|task| {
                (
                    task.id,
                    Task {
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
                    },
                )
            })
            .collect();

        //println!("{:?}", tasks);
        Ok(tasks_map)
    }
}
