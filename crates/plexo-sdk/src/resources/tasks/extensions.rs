use std::str::FromStr;

use async_graphql::InputObject;
use async_trait::async_trait;
use derive_builder::Builder;
use serde::Serialize;
use sqlx::Row;
use uuid::Uuid;

use crate::{backend::engine::SDKEngine, errors::sdk::SDKError};

use super::{
    operations::{CreateTaskInput, TaskCrudOperations},
    task::{Task, TaskPriority, TaskStatus},
};

#[derive(Default, Builder, InputObject, Clone, Serialize)]
#[builder(pattern = "owned")]
pub struct CreateTasksInput {
    pub tasks: Vec<CreateTaskInput>,
}

#[async_trait]
pub trait TasksExtensionOperations {
    async fn create_tasks(&self, input: CreateTasksInput) -> Result<Vec<Task>, SDKError>;
}

#[async_trait]
impl TasksExtensionOperations for SDKEngine {
    async fn create_tasks(&self, input: CreateTasksInput) -> Result<Vec<Task>, SDKError> {
        let mut tx = self.db_pool.begin().await?;
        // let saved_input = input.clone();

        let values = input
            .tasks
            .iter()
            .map(|task| {
                format!(
                    "('{}', '{}', {}, '{}', '{}', {}, {}, {}, {})",
                    task.title,
                    task.owner_id,
                    task.description
                        .clone()
                        .map(|d| format!("'{}'", d))
                        .unwrap_or("null".to_string()),
                    task.status.unwrap_or_default(),
                    task.priority.unwrap_or_default(),
                    task.due_date
                        .map(|dd| format!("'{}'", dd))
                        .unwrap_or("null".to_string()),
                    task.project_id
                        .map(|p| format!("'{}'", p))
                        .unwrap_or("null".to_string()),
                    task.lead_id.map(|l| format!("'{}'", l)).unwrap_or("null".to_string()),
                    task.parent_id.map(|p| format!("'{}'", p)).unwrap_or("null".to_string()),
                )
            })
            .collect::<Vec<String>>();

        let query = format!(
            "INSERT INTO tasks (title, owner_id, description, status, priority, due_date, project_id, lead_id, parent_id) VALUES {} RETURNING *",
            values.join(", ")
        );

        let tasks = sqlx::query(query.as_str()).fetch_all(&mut *tx).await?;

        for (i, input_task) in input.tasks.iter().enumerate() {
            let task = &tasks[i];

            let task_id = task.get::<Uuid, _>("id");
            let task_owner_id = task.get::<Uuid, _>("owner_id");

            if let Some(labels) = input_task.labels.clone() {
                for label in labels {
                    sqlx::query!(
                        r#"
                    INSERT INTO labels_by_tasks (task_id, label_id)
                    VALUES ($1, $2)
                    "#,
                        task_id,
                        label,
                    )
                    .execute(&mut *tx)
                    .await?;
                }
            }

            if let Some(assignees) = input_task.assignees.clone() {
                for assignee in assignees {
                    sqlx::query!(
                        r#"
                        INSERT INTO tasks_by_assignees (task_id, assignee_id)
                        VALUES ($1, $2)
                        "#,
                        task_id,
                        assignee,
                    )
                    .execute(&mut *tx)
                    .await?;
                }
            }

            if let Some(subtasks) = input_task.subtasks.clone() {
                for mut subtask in subtasks {
                    subtask.owner_id = task_owner_id;

                    if subtask.parent_id.is_none() {
                        subtask.parent_id = Some(task_id);
                    }

                    self.create_task(subtask).await?;
                }
            }
        }

        tx.commit().await?;

        let tasks: Vec<Task> = tasks
            .iter()
            .map(|task_info| Task {
                id: task_info.get("id"),
                created_at: task_info.get("created_at"),
                updated_at: task_info.get("updated_at"),
                title: task_info.get("title"),
                description: task_info.get("description"),
                status: task_info
                    .get::<'_, Option<String>, _>("status")
                    .and_then(|a| TaskStatus::from_str(&a).ok())
                    .unwrap_or_default(),
                priority: task_info
                    .get::<'_, Option<String>, _>("priority")
                    .and_then(|a| TaskPriority::from_str(&a).ok())
                    .unwrap_or_default(),
                due_date: task_info.get("due_date"),
                project_id: task_info.get("project_id"),
                lead_id: task_info.get("lead_id"),
                owner_id: task_info.get("owner_id"),
                count: task_info.get("count"),
                parent_id: task_info.get("parent_id"),
            })
            .collect();

        // if self.config.with_changes_registration {
        //     let tasks = tasks.clone();
        //     let engine = self.clone();

        //     task::spawn(async move {
        //         for task in tasks {
        //             let change = engine
        //                 .create_change(
        //                     CreateChangeInputBuilder::default()
        //                         .owner_id(task.owner_id)
        //                         .resource_id(task.id)
        //                         .operation(ChangeOperation::Create)
        //                         .resource_type(ChangeResourceType::Task)
        //                         .diff_json(
        //                             serde_json::to_string(&json!({
        //                                 "input": saved_input,
        //                                 "result": task,
        //                             }))
        //                             .unwrap(),
        //                         )
        //                         .build()
        //                         .unwrap(),
        //                 )
        //                 .await
        //                 .unwrap();

        //             println!("change registered 2: {} | {}", change.operation, change.resource_type);
        //         }
        //     });
        // }

        Ok(tasks)
    }
}
