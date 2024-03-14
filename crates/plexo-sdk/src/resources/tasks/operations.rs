use std::str::FromStr;

use async_graphql::InputObject;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

use derive_builder::Builder;
use poem_openapi::Object;
use serde::Serialize;
// use serde_json::json;
use sqlx::Row;
// use tokio::task;
use uuid::Uuid;

use crate::backend::engine::SDKEngine;
use crate::common::commons::{SortOrder, UpdateListInput};
use crate::errors::sdk::SDKError;
// use crate::resources::changes::change::{ChangeOperation, ChangeResourceType};
// use crate::resources::changes::operations::{ChangeCrudOperations, CreateChangeInputBuilder};
use crate::resources::tasks::task::{Task, TaskPriority, TaskStatus};

#[async_trait]
pub trait TaskCrudOperations {
    async fn create_task(&self, input: CreateTaskInput) -> Result<Task, SDKError>;
    async fn get_task(&self, id: Uuid) -> Result<Task, SDKError>;
    async fn get_tasks(&self, input: Option<GetTasksInput>) -> Result<Vec<Task>, SDKError>;
    async fn update_task(&self, id: Uuid, input: UpdateTaskInput) -> Result<Task, SDKError>;
    async fn delete_task(&self, id: Uuid) -> Result<Task, SDKError>;
}

#[derive(Default, Builder, Object, InputObject)]
#[builder(pattern = "owned")]
pub struct GetTasksInput {
    #[builder(setter(strip_option), default)]
    pub filter: Option<GetTasksWhere>,

    #[builder(setter(strip_option), default)]
    pub sort_by: Option<String>,
    #[builder(setter(strip_option), default)]
    pub sort_order: Option<SortOrder>,

    #[builder(setter(into, strip_option), default = "Some(100)")]
    pub limit: Option<i32>,
    #[builder(setter(into, strip_option), default = "Some(0)")]
    pub offset: Option<i32>,
}

#[derive(Clone, Default, Builder, Object, InputObject, Serialize)]
#[builder(pattern = "owned")]
pub struct CreateTaskInput {
    pub title: String,

    #[graphql(skip)]
    pub owner_id: Uuid,

    #[builder(setter(strip_option), default)]
    pub status: Option<TaskStatus>,
    #[builder(setter(strip_option), default)]
    pub priority: Option<TaskPriority>,

    #[builder(setter(strip_option), default)]
    pub description: Option<String>,
    #[builder(setter(strip_option), default)]
    pub due_date: Option<DateTime<Utc>>,
    #[builder(setter(strip_option), default)]
    pub project_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub lead_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub parent_id: Option<Uuid>,

    #[builder(setter(strip_option), default)]
    pub labels: Option<Vec<Uuid>>,
    #[builder(setter(strip_option), default)]
    pub assignees: Option<Vec<Uuid>>,
    #[oai(skip)]
    #[builder(setter(strip_option), default)]
    pub subtasks: Option<Vec<CreateTaskInput>>,

    #[builder(setter(strip_option), default)]
    pub assets: Option<Vec<Uuid>>,
}

#[derive(Default, Builder, Object, InputObject, Serialize, Clone)]
#[builder(pattern = "owned")]
pub struct UpdateTaskInput {
    #[builder(setter(strip_option), default)]
    pub status: Option<TaskStatus>,
    #[builder(setter(strip_option), default)]
    pub priority: Option<TaskPriority>,
    #[builder(setter(strip_option), default)]
    pub title: Option<String>,
    #[builder(setter(strip_option), default)]
    pub description: Option<String>,
    #[builder(setter(strip_option), default)]
    pub due_date: Option<DateTime<Utc>>,
    #[builder(setter(strip_option), default)]
    pub project_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub lead_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub parent_id: Option<Uuid>,

    #[builder(setter(strip_option), default)]
    pub labels: Option<UpdateListInput>,
    #[builder(setter(strip_option), default)]
    pub assignees: Option<UpdateListInput>,
    #[builder(setter(strip_option), default)]
    pub assets: Option<UpdateListInput>,
}

#[derive(Default, Builder, Object, InputObject)]
#[builder(pattern = "owned")]
pub struct GetTasksWhere {
    #[builder(setter(strip_option), default)]
    pub ids: Option<Vec<Uuid>>,
    #[builder(setter(strip_option), default)]
    pub owner_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub status: Option<TaskStatus>,
    #[builder(setter(strip_option), default)]
    pub priority: Option<TaskPriority>,
    #[builder(setter(strip_option), default)]
    pub title: Option<String>,
    #[builder(setter(strip_option), default)]
    pub description: Option<String>,
    #[builder(setter(strip_option), default)]
    pub due_date: Option<DateTime<Utc>>,
    #[builder(setter(strip_option), default)]
    pub project_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub lead_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub parent_id: Option<Uuid>,

    #[oai(skip)]
    #[builder(setter(strip_option), default)]
    pub _and: Option<Vec<GetTasksWhere>>,

    #[oai(skip)]
    #[builder(setter(strip_option), default)]
    pub _or: Option<Vec<GetTasksWhere>>,
}

impl GetTasksWhere {
    pub fn compile_sql(&self) -> String {
        let mut conditions = Vec::new();

        if let Some(ids) = &self.ids {
            conditions.push(format!(
                "id = ANY(array[{}]::uuid[])",
                ids.iter()
                    .map(|x| format!("'{}'", x))
                    .collect::<Vec<String>>()
                    .join(",")
            ));
        }

        if let Some(owner_id) = &self.owner_id {
            conditions.push(format!("owner_id = '{}'", owner_id));
        }

        if let Some(status) = &self.status {
            conditions.push(format!("status = '{}'", status));
        }

        if let Some(priority) = &self.priority {
            conditions.push(format!("priority = '{}'", priority));
        }

        if let Some(title) = &self.title {
            conditions.push(format!("title = '{}'", title));
        }

        if let Some(description) = &self.description {
            conditions.push(format!("description = '{}'", description));
        }

        if let Some(due_date) = &self.due_date {
            conditions.push(format!("due_date = '{}'", due_date));
        }

        if let Some(project_id) = &self.project_id {
            conditions.push(format!("project_id = '{}'", project_id));
        }

        if let Some(lead_id) = &self.lead_id {
            conditions.push(format!("lead_id = '{}'", lead_id));
        }

        if let Some(parent_id) = &self.parent_id {
            conditions.push(format!("parent_id = '{}'", parent_id));
        }

        if let Some(ands) = &self._and {
            let and_conditions: Vec<String> = ands.iter().map(|and| and.compile_sql()).collect();
            conditions.push(format!("({})", and_conditions.join(" AND ")));
        }

        if let Some(ors) = &self._or {
            let or_conditions: Vec<String> = ors.iter().map(|or| or.compile_sql()).collect();
            conditions.push(format!("({})", or_conditions.join(" OR ")));
        }

        conditions.join(" AND ")
    }
}

#[async_trait]
impl TaskCrudOperations for SDKEngine {
    async fn create_task(&self, input: CreateTaskInput) -> Result<Task, SDKError> {
        let mut tx = self.db_pool.begin().await?;
        // let saved_input = input.clone();

        let task = sqlx::query!(
            r#"
            INSERT INTO tasks (title, description, owner_id, status, priority, due_date, project_id, lead_id, parent_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
            input.title,
            input.description,
            input.owner_id,
            input.status.unwrap_or_default().to_string(),
            input.priority.unwrap_or_default().to_string(),
            input.due_date,
            input.project_id,
            input.lead_id,
            input.parent_id,
        )
        .fetch_one(&mut *tx)
        .await?;

        if let Some(labels) = input.labels {
            for label in labels {
                sqlx::query!(
                    r#"
                    INSERT INTO labels_by_tasks (task_id, label_id)
                    VALUES ($1, $2)
                    "#,
                    task.id,
                    label,
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        if let Some(assignees) = input.assignees {
            for assignee in assignees {
                sqlx::query!(
                    r#"
                    INSERT INTO tasks_by_assignees (task_id, assignee_id)
                    VALUES ($1, $2)
                    "#,
                    task.id,
                    assignee,
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        if let Some(subtasks) = input.subtasks {
            for mut subtask in subtasks {
                if subtask.parent_id.is_none() {
                    subtask.parent_id = Some(task.id);
                }

                self.create_task(subtask).await?;
            }
        }

        // if let Some(assets) = input.assets {
        //     for asset in assets {
        //         sqlx::query!(
        //             r#"
        //             INSERT INTO assets_by_tasks (task_id, asset_id)
        //             VALUES ($1, $2)
        //             "#,
        //             task.id,
        //             asset,
        //         )
        //         .execute(&mut *tx)
        //         .await?;
        //     }
        // }

        tx.commit().await?;

        let task = Task {
            id: task.id,
            created_at: task.created_at,
            updated_at: task.updated_at,
            title: task.title,
            description: task.description,
            status: task
                .status
                .and_then(|a| TaskStatus::from_str(&a).ok())
                .unwrap_or_default(),
            priority: task
                .priority
                .and_then(|a| TaskPriority::from_str(&a).ok())
                .unwrap_or_default(),
            due_date: task.due_date,
            project_id: task.project_id,
            lead_id: task.lead_id,
            owner_id: task.owner_id,
            count: task.count,
            parent_id: task.parent_id,
        };

        // if self.config.with_changes_registration {
        //     let input = saved_input.clone();
        //     let task = task.clone();
        //     let engine = self.clone();

        //     task::spawn(async move {
        //         let change = engine
        //             .create_change(
        //                 CreateChangeInputBuilder::default()
        //                     .owner_id(task.owner_id)
        //                     .resource_id(task.id)
        //                     .operation(ChangeOperation::Create)
        //                     .resource_type(ChangeResourceType::Task)
        //                     .diff_json(
        //                         serde_json::to_string(&json!({
        //                             "input": input,
        //                             "result": task,
        //                         }))
        //                         .unwrap(),
        //                     )
        //                     .build()
        //                     .unwrap(),
        //             )
        //             .await
        //             .unwrap();

        //         println!("change registered 1: {} | {}", change.operation, change.resource_type);
        //     });
        // }

        Ok(task)
    }

    async fn get_task(&self, id: Uuid) -> Result<Task, SDKError> {
        let task_info = sqlx::query!(
            r#"
            SELECT * FROM tasks WHERE id = $1
            "#,
            id,
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        let task = Task {
            id: task_info.id,
            created_at: task_info.created_at,
            updated_at: task_info.updated_at,
            title: task_info.title,
            description: task_info.description,
            status: task_info
                .status
                .and_then(|a| TaskStatus::from_str(&a).ok())
                .unwrap_or_default(),
            priority: task_info
                .priority
                .and_then(|a| TaskPriority::from_str(&a).ok())
                .unwrap_or_default(),
            due_date: task_info.due_date,
            project_id: task_info.project_id,
            lead_id: task_info.lead_id,
            owner_id: task_info.owner_id,
            count: task_info.count,
            parent_id: task_info.parent_id,
        };

        Ok(task)
    }

    async fn update_task(&self, id: Uuid, input: UpdateTaskInput) -> Result<Task, SDKError> {
        let mut tx = self.db_pool.begin().await?;

        let task_final_info = sqlx::query!(
            r#"
            UPDATE tasks
            SET
                status = COALESCE($1, status),
                priority = COALESCE($2, priority),
                title = COALESCE($3, title),
                description = COALESCE($4, description),
                due_date = COALESCE($5, due_date),
                project_id = NULLIF(COALESCE($6, project_id), '00000000-0000-0000-0000-000000000000'),
                lead_id = NULLIF(COALESCE($7, lead_id), '00000000-0000-0000-0000-000000000000'),
                parent_id = NULLIF(COALESCE($8, parent_id), '00000000-0000-0000-0000-000000000000')
            WHERE id = $9
            RETURNING *
            "#,
            input.status.map(|status| status.to_string()),
            input.priority.map(|priority| priority.to_string()),
            input.title,
            input.description,
            input.due_date,
            input.project_id,
            input.lead_id,
            input.parent_id,
            id,
        )
        .fetch_one(&mut *tx)
        .await?;

        if let Some(labels) = input.labels {
            for label in labels.add {
                sqlx::query!(
                    r#"
                    INSERT INTO labels_by_tasks (task_id, label_id)
                    VALUES ($1, $2)
                    "#,
                    id,
                    label,
                )
                .execute(&mut *tx)
                .await?;
            }

            for label in labels.remove {
                sqlx::query!(
                    r#"
                    DELETE FROM labels_by_tasks WHERE task_id = $1 AND label_id = $2
                    "#,
                    id,
                    label,
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        if let Some(assignees) = input.assignees {
            for assignee in assignees.add {
                sqlx::query!(
                    r#"
                    INSERT INTO tasks_by_assignees (task_id, assignee_id)
                    VALUES ($1, $2)
                    "#,
                    id,
                    assignee,
                )
                .execute(&mut *tx)
                .await?;
            }

            for assignee in assignees.remove {
                sqlx::query!(
                    r#"
                    DELETE FROM tasks_by_assignees WHERE task_id = $1 AND assignee_id = $2
                    "#,
                    id,
                    assignee,
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;

        // if let Some(assets) = input.assets {
        //     for asset in assets.add {
        //         sqlx::query!(
        //             r#"
        //             INSERT INTO assets_by_tasks (task_id, asset_id)
        //             VALUES ($1, $2)
        //             "#,
        //             id,
        //             asset,
        //         )
        //         .execute(self.db_pool.as_ref())
        //         .await?;
        //     }

        //     for asset in assets.remove {
        //         sqlx::query!(
        //             r#"
        //             DELETE FROM assets_by_tasks WHERE task_id = $1 AND asset_id = $2
        //             "#,
        //             id,
        //             asset,
        //         )
        //         .execute(self.db_pool.as_ref())
        //         .await?;
        //     }
        // }

        let task = Task {
            id: task_final_info.id,
            created_at: task_final_info.created_at,
            updated_at: task_final_info.updated_at,
            title: task_final_info.title,
            description: task_final_info.description,
            status: task_final_info
                .status
                .and_then(|a| TaskStatus::from_str(&a).ok())
                .unwrap_or_default(),
            priority: task_final_info
                .priority
                .and_then(|a| TaskPriority::from_str(&a).ok())
                .unwrap_or_default(),
            due_date: task_final_info.due_date,
            project_id: task_final_info.project_id,
            lead_id: task_final_info.lead_id,
            owner_id: task_final_info.owner_id,
            count: task_final_info.count,
            parent_id: task_final_info.parent_id,
        };

        // if self.config.with_changes_registration {
        //     let task = task.clone();
        //     let engine = self.clone();

        //     tokio::spawn(async move {
        //         engine
        //             .create_change(
        //                 CreateChangeInputBuilder::default()
        //                     .owner_id(task.owner_id)
        //                     .resource_id(task.id)
        //                     .operation(ChangeOperation::Update)
        //                     .resource_type(ChangeResourceType::Task)
        //                     .diff_json(
        //                         serde_json::to_string(&json!({
        //                             "input": saved_input,
        //                             "result": task,
        //                         }))
        //                         .unwrap(),
        //                     )
        //                     .build()
        //                     .unwrap(),
        //             )
        //             .await
        //             .unwrap();
        //     });
        // }

        Ok(task)
    }

    async fn delete_task(&self, id: Uuid) -> Result<Task, SDKError> {
        let task_info = sqlx::query!(
            r#"
            DELETE FROM tasks WHERE id = $1
            RETURNING *
            "#,
            id,
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        let task = Task {
            id: task_info.id,
            created_at: task_info.created_at,
            updated_at: task_info.updated_at,
            title: task_info.title,
            description: task_info.description,
            status: task_info
                .status
                .and_then(|a| TaskStatus::from_str(&a).ok())
                .unwrap_or_default(),
            priority: task_info
                .priority
                .and_then(|a| TaskPriority::from_str(&a).ok())
                .unwrap_or_default(),
            due_date: task_info.due_date,
            project_id: task_info.project_id,
            lead_id: task_info.lead_id,
            owner_id: task_info.owner_id,
            count: task_info.count,
            parent_id: task_info.parent_id,
        };

        // if self.config.with_changes_registration {
        //     let task = task.clone();
        //     let engine = self.clone();

        //     tokio::spawn(async move {
        //         let change = engine
        //             .create_change(
        //                 CreateChangeInputBuilder::default()
        //                     .owner_id(task.owner_id)
        //                     .resource_id(task.id)
        //                     .operation(ChangeOperation::Delete)
        //                     .resource_type(ChangeResourceType::Task)
        //                     .diff_json(
        //                         serde_json::to_string(&json!({
        //                             "result": task,
        //                         }))
        //                         .unwrap(),
        //                     )
        //                     .build()
        //                     .unwrap(),
        //             )
        //             .await
        //             .unwrap();

        //         println!("change registered: {} | {}", change.operation, change.resource_type);
        //     });
        // }

        Ok(task)
    }

    async fn get_tasks(&self, input: Option<GetTasksInput>) -> Result<Vec<Task>, SDKError> {
        let mut query = "SELECT * FROM tasks ".to_string();

        let query = match input {
            Some(input) => {
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

                query
            }
            None => query,
        };

        let tasks_info = sqlx::query(query.as_str()).fetch_all(self.db_pool.as_ref()).await?;

        Ok(tasks_info
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
                // status: TaskStatus::from_optional_str(&task_info.get("status")),
                // priority: TaskPriority::from_optional_str(&),
                due_date: task_info.get("due_date"),
                project_id: task_info.get("project_id"),
                lead_id: task_info.get("lead_id"),
                owner_id: task_info.get("owner_id"),
                count: task_info.get("count"),
                parent_id: task_info.get("parent_id"),
            })
            .collect())
    }
}
