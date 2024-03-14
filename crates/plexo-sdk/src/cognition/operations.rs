use async_graphql::{InputObject, SimpleObject};
use async_trait::async_trait;
use chrono::{DateTime, Local, Utc};
use derive_builder::Builder;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    backend::engine::SDKEngine,
    errors::sdk::SDKError,
    resources::tasks::{
        operations::TaskCrudOperations,
        task::{TaskPriority, TaskStatus},
    },
};

use super::suggestions::CognitionCapabilities;

#[derive(Default, Builder, Object, InputObject, Serialize)]
#[builder(pattern = "owned")]
pub struct TaskSuggestionInput {
    #[builder(setter(strip_option), default)]
    pub project_id: Option<Uuid>,

    #[builder(setter(strip_option), default)]
    pub title: Option<String>,
    #[builder(setter(strip_option), default)]
    pub description: Option<String>,
    #[builder(setter(strip_option), default)]
    pub status: Option<TaskStatus>,
    #[builder(setter(strip_option), default)]
    pub priority: Option<TaskPriority>,
    #[builder(setter(strip_option), default)]
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Default, Builder, Object, SimpleObject, Deserialize)]
#[builder(pattern = "owned")]
pub struct TaskSuggestion {
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub due_date: DateTime<Utc>,
}

#[derive(Default, Builder, Object, InputObject)]
#[builder(pattern = "owned")]
pub struct SubdivideTaskInput {
    pub task_id: Uuid,
    pub subtasks: u8, // TODO: validate it or die

    #[builder(setter(strip_option), default)]
    pub with_tasks_context: Option<bool>,
}

#[async_trait]
pub trait CognitionOperations {
    async fn get_suggestions(&self, input: TaskSuggestionInput) -> Result<TaskSuggestion, SDKError>;
    async fn subdivide_task(&self, input: SubdivideTaskInput) -> Result<Vec<TaskSuggestion>, SDKError>;
}

#[async_trait]
impl CognitionOperations for SDKEngine {
    async fn get_suggestions(&self, input: TaskSuggestionInput) -> Result<TaskSuggestion, SDKError> {
        let tasks_fingerprints = self.acquire_tasks_fingerprints(10, input.project_id).await;

        let system_message =
            "The user pass to you a list of tasks and you should predict the following based on the input of the user.
        Please return only a valid json with the following struct {
                title: String,
                description: String,
                status: TaskStatus,
                priority: TaskPriority,
                due_date: DateTime<Utc>
        }"
            .to_string();

        let user_message = format!(
            "
            Current Time:
            {}

            Current Tasks Context: 
            {}
            
            With the above context, complete the following task, only fill the <suggest> fields:
            {}",
            Local::now(),
            tasks_fingerprints.join("\n\n"),
            Self::calculate_task_suggestion_fingerprint(input),
        );

        let result = self.chat_completion(system_message, user_message).await;
        let result = result.trim().trim_matches('`');

        let suggestion_result: TaskSuggestion = serde_json::from_str(result)?;

        Ok(suggestion_result)
    }

    async fn subdivide_task(&self, input: SubdivideTaskInput) -> Result<Vec<TaskSuggestion>, SDKError> {
        let task = self.get_task(input.task_id).await?;

        let system_message = "The user pass to you one task and you should predict a list of subtasks.
        Please return only a valid json with the following struct [{
                title: String,
                description: String,
                status: TaskStatus,
                priority: TaskPriority,
                due_date: DateTime<Utc>
        }]
        For TaskStatus and TaskPriority, please use the following values:
        TaskStatus: None, Backlog, ToDo, InProgress, Done, Canceled
        TaskPriority: None, Low, Medium, High, Urgent
        "
        .to_string();

        let user_message = format!(
            "
            Current Time:
            {}

            Parent Task: 
            {}
            
            With the above context, generate {} subtasks.",
            Local::now(),
            Self::calculate_task_fingerprint(task),
            input.subtasks,
        );

        let result = self.chat_completion(system_message, user_message).await;
        let result = result.trim().trim_matches('`');

        let subtasks: Vec<TaskSuggestion> = serde_json::from_str(result)?;

        Ok(subtasks)
    }
}

// impl CognitionOperations for SDKEngine {
//     async fn get_suggestions(&self, input: TaskSuggestionInput) -> Result<TaskSuggestion, SDKError> {
//         let tasks_fingerprints = self.acquire_tasks_fingerprints(10, input.project_id).await;

//         let system_message =
//             "Based on the task information provided by the user, predict the next task including title, description, status, priority, and due date. Ensure the response is in valid JSON format matching the TaskSuggestion structure. Use TaskStatus values: None, Backlog, ToDo, InProgress, Done, Canceled; and TaskPriority values: None, Low, Medium, High, Urgent."
//             .to_string();

//         let user_message = format!(
//             "Given the current time: {}
//             and the context of current tasks: {}
//             Generate a task suggestion based on the user input below, ensuring all fields are filled appropriately in JSON format.",
//             Local::now(),
//             tasks_fingerprints.join("\n\n"),
//         );

//         let result = self.chat_completion(system_message, user_message).await;
//         let result = result.trim().trim_matches('`');

//         let suggestion_result: TaskSuggestion = serde_json::from_str(result)?;

//         Ok(suggestion_result)
//     }

//     async fn subdivide_task(&self, input: SubdivideTaskInput) -> Result<Vec<TaskSuggestion>, SDKError> {
//         let task = self.get_task(input.task_id).await?;

//         let system_message = "Given a single task, generate a list of subtasks in a valid JSON format, including details such as title, description, status, priority, and due date. Use specific values for TaskStatus and TaskPriority as mentioned previously."
//         .to_string();

//         let user_message = format!(
//             "Considering the current time: {}
//             and the details of the parent task: {}
//             Generate {} subtasks in JSON format, adhering to the specified structure for each subtask.",
//             Local::now(),
//             Self::calculate_task_fingerprint(task),
//             input.subtasks,
//         );

//         let result = self.chat_completion(system_message, user_message).await;
//         let result = result.trim().trim_matches('`');

//         let subtasks: Vec<TaskSuggestion> = serde_json::from_str(result)?;

//         Ok(subtasks)
//     }
// }
