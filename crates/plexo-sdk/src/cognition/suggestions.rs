use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use async_trait::async_trait;

use uuid::Uuid;

use super::operations::TaskSuggestionInput;
use crate::{
    backend::engine::SDKEngine,
    resources::tasks::{
        operations::{GetTasksInputBuilder, TaskCrudOperations},
        task::Task,
    },
};

#[async_trait]
pub trait CognitionCapabilities {
    async fn chat_completion(&self, system_message: String, user_message: String) -> String;
    async fn acquire_tasks_fingerprints(&self, number_of_tasks: u32, project_id: Option<Uuid>) -> Vec<String>;

    fn calculate_task_fingerprint(task: Task) -> String;
    fn calculate_task_suggestion_fingerprint(task_suggestion: TaskSuggestionInput) -> String;
}

#[async_trait]
impl CognitionCapabilities for SDKEngine {
    async fn chat_completion(&self, system_message: String, user_message: String) -> String {
        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(1024u16)
            .model(self.config.llm_model_name.clone())
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_message)
                    .build()
                    .unwrap()
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user_message)
                    .build()
                    .unwrap()
                    .into(),
            ])
            .build()
            .unwrap();

        let response = self.llm_client.chat().create(request).await.unwrap();

        response.choices.first().unwrap().message.content.clone().unwrap()
    }

    fn calculate_task_fingerprint(task: Task) -> String {
        serde_json::to_string(&task).unwrap()
    }

    fn calculate_task_suggestion_fingerprint(task_suggestion: TaskSuggestionInput) -> String {
        format!(
            "Task Title: {}
        Task Description: {}
        Task Status: {}
        Task Priority: {}
        Task Due Date: {}",
            task_suggestion.title.unwrap_or("<suggest>".to_string()),
            task_suggestion.description.unwrap_or("<suggest>".to_string()),
            task_suggestion
                .status
                .map(|s| s.to_string())
                .unwrap_or("<suggest>".to_string()),
            task_suggestion
                .priority
                .map(|p| p.to_string())
                .unwrap_or("<suggest>".to_string()),
            task_suggestion
                .due_date
                .map(|d| d.to_rfc3339())
                .unwrap_or("<suggest>".to_string()),
        )
    }

    async fn acquire_tasks_fingerprints(&self, number_of_tasks: u32, _project_id: Option<Uuid>) -> Vec<String> {
        let filter = GetTasksInputBuilder::default()
            .limit(number_of_tasks as i32)
            .build()
            .ok();

        let tasks = self.get_tasks(filter).await.unwrap();

        tasks
            .iter()
            .map(|r| Task {
                id: r.id,
                created_at: r.created_at,
                updated_at: r.updated_at,
                title: r.title.clone(),
                description: r.description.clone(),
                status: r.status,
                priority: r.priority,
                due_date: r.due_date,
                project_id: r.project_id,
                lead_id: r.lead_id,
                owner_id: r.owner_id,
                count: r.count,
                parent_id: r.parent_id,
            })
            .map(Self::calculate_task_fingerprint)
            .collect::<Vec<String>>()
    }
}
