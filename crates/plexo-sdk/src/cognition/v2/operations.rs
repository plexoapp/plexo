use std::pin::Pin;

use askama::Template;
use async_stream::stream;
use async_trait::async_trait;

use serde_json::json;
use tokio_stream::{Stream, StreamExt};

use crate::{
    backend::engine::SDKEngine,
    cognition::{
        operations::{SubdivideTaskInput, TaskSuggestion, TaskSuggestionInput},
        suggestions::CognitionCapabilities,
    },
    common::commons::SortOrder,
    errors::sdk::SDKError,
    organization::operations::{Organization, OrganizationCrudOperations},
    resources::{
        chats::operations::ChatCrudOperations,
        members::{
            member::Member,
            operations::{GetMembersInputBuilder, MemberCrudOperations},
        },
        messages::operations::{
            CreateMessageInputBuilder, GetMessagesInputBuilder, GetMessagesWhereBuilder, MessageCrudOperations,
        },
        projects::{
            operations::{GetProjectsInputBuilder, ProjectCrudOperations},
            project::Project,
        },
        tasks::{
            operations::{GetTasksInputBuilder, GetTasksWhereBuilder, TaskCrudOperations},
            task::Task,
        },
        teams::{
            operations::{GetTeamsInputBuilder, TeamCrudOperations},
            team::Team,
        },
    },
};

use super::{
    chat::{ChatResponseChunk, ChatResponseInput},
    projects::{ProjectSuggestion, ProjectSuggestionInput, ProjectTaskSuggestionInput},
};

#[async_trait]
pub trait CognitionOperationsV2 {
    async fn get_suggestions_v2(&self, input: TaskSuggestionInput) -> Result<TaskSuggestion, SDKError>;
    async fn subdivide_task_v2(&self, input: SubdivideTaskInput) -> Result<Vec<TaskSuggestion>, SDKError>;
    async fn get_project_suggestion(&self, input: ProjectSuggestionInput) -> Result<ProjectSuggestion, SDKError>;
    async fn get_chat_response(
        &self,
        input: ChatResponseInput,
    ) -> Result<Pin<Box<dyn Stream<Item = ChatResponseChunk> + Send>>, SDKError>;
}

fn calculate_task_fingerprint(task: &Task) -> String {
    serde_json::to_string_pretty(&task).unwrap()
}

fn calculate_project_fingerprint(project: &Project) -> String {
    serde_json::to_string_pretty(&project).unwrap()
}

fn calculate_organization_fingerprint(project: &Organization) -> String {
    serde_json::to_string_pretty(&project).unwrap()
}

fn calculate_member_fingerprint(project: &Member) -> String {
    serde_json::to_string_pretty(&project).unwrap()
}

fn calculate_team_fingerprint(project: &Team) -> String {
    serde_json::to_string_pretty(&project).unwrap()
}

fn calculate_task_suggestion_input_fingerprint(input: &TaskSuggestionInput) -> String {
    serde_json::to_string_pretty(&input).unwrap()
}

fn current_time() -> String {
    chrono::Local::now().to_string()
}

fn calculate_project_suggestion_input_fingerprint(input: &ProjectSuggestionInput) -> String {
    serde_json::to_string_pretty(&input).unwrap()
}

fn calculate_task_suggestion_fingerprint(input: &ProjectTaskSuggestionInput) -> String {
    serde_json::to_string_pretty(&input).unwrap()
}

#[derive(Template)]
#[template(path = "task_suggestion.md.jinja", ext = "plain")]
pub struct TaskSuggestionTemplate {
    tasks: Vec<Task>,
    initial_state: Option<TaskSuggestionInput>,
    project: Option<Project>,
    user_query: Option<String>,
}

#[derive(Template)]
#[template(path = "task_subdivide.md.jinja", ext = "plain")]
pub struct TaskSubdivideTemplate {
    parent_task: Task,
    number_of_subtasks: u8,
    project: Option<Project>,
    tasks: Option<Vec<Task>>,
    user_query: Option<String>,
}

#[derive(Template)]
#[template(path = "plexo_system.md.jinja", ext = "plain")]
pub struct PlexoSystemTemplate {}

#[derive(Template)]
#[template(path = "project_suggestion.md.jinja", ext = "plain")]
pub struct ProjectSuggestionTemplate {
    title: String,
    generate_tasks_number: u8,
    projects: Vec<Project>,
    initial_state: Option<ProjectSuggestionInput>,
    initial_tasks: Option<Vec<ProjectTaskSuggestionInput>>,
    user_query: Option<String>,
}

#[derive(Template)]
#[template(path = "project_related_chat.md.jinja", ext = "plain")]
pub struct ProjectRelatedChatTemplate {
    project: Project,
    tasks: Vec<Task>,
}

#[derive(Template)]
#[template(path = "organization_related_chat.md.jinja", ext = "plain")]
pub struct OrganizationRelatedChatTemplate {
    organization: Organization,
    projects: Vec<Project>,
    tasks: Vec<Task>,
    members: Vec<Member>,
    teams: Vec<Team>,
}

#[async_trait]
impl CognitionOperationsV2 for SDKEngine {
    async fn get_suggestions_v2(&self, input: TaskSuggestionInput) -> Result<TaskSuggestion, SDKError> {
        let system_message = PlexoSystemTemplate {}.render().unwrap();

        let (tasks, project) = match input.project_id {
            Some(project_id) => {
                let project = self.get_project(project_id).await?;

                (
                    self.get_tasks(
                        GetTasksInputBuilder::default()
                            .filter(GetTasksWhereBuilder::default().project_id(project_id).build().unwrap())
                            .sort_by("created_at".to_string())
                            .sort_order(SortOrder::Asc)
                            .limit(10)
                            .build()
                            .ok(),
                    )
                    .await?,
                    Some(project),
                )
            }

            None => (
                self.get_tasks(
                    GetTasksInputBuilder::default()
                        .sort_by("created_at".to_string())
                        .sort_order(SortOrder::Asc)
                        .limit(10)
                        .build()
                        .ok(),
                )
                .await?,
                None,
            ),
        };

        let input_message = TaskSuggestionTemplate {
            tasks,
            project,
            initial_state: Some(input),
            user_query: None,
        }
        .render()
        .unwrap();

        let result = self.chat_completion(system_message, input_message).await;
        let result = result.trim().trim_matches('`');

        let suggestion_result: TaskSuggestion = serde_json::from_str(result).inspect_err(|err| {
            println!("Error parsing suggestion result: {:?}", err);
            println!("raw result: {:?}", result);
        })?;

        Ok(suggestion_result)
    }

    async fn subdivide_task_v2(&self, input: SubdivideTaskInput) -> Result<Vec<TaskSuggestion>, SDKError> {
        let system_message = PlexoSystemTemplate {}.render().unwrap();

        let parent_task = self.get_task(input.task_id).await?;

        let (project, project_id) = match parent_task.project_id {
            Some(project_id) => (Some(self.get_project(project_id).await?), Some(project_id)),
            None => (None, None),
        };

        let tasks = match (input.with_tasks_context, project_id) {
            (Some(true), Some(project_id)) => {
                let tasks = self
                    .get_tasks(
                        GetTasksInputBuilder::default()
                            .filter(GetTasksWhereBuilder::default().project_id(project_id).build().unwrap())
                            .sort_by("created_at".to_string())
                            .sort_order(SortOrder::Desc)
                            .limit(10)
                            .build()
                            .ok(),
                    )
                    .await?;

                Some(tasks)
            }
            (Some(true), None) => {
                let tasks = self
                    .get_tasks(
                        GetTasksInputBuilder::default()
                            .sort_by("created_at".to_string())
                            .sort_order(SortOrder::Desc)
                            .limit(10)
                            .build()
                            .ok(),
                    )
                    .await?;

                Some(tasks)
            }
            (None, _) | (Some(false), _) => None,
        };

        let input_message = TaskSubdivideTemplate {
            parent_task,
            number_of_subtasks: input.subtasks,
            project,
            tasks,
            user_query: None,
        }
        .render()
        .unwrap();

        let result = self.chat_completion(system_message, input_message).await;
        let result = result.trim().trim_matches('`');

        let subtasks: Vec<TaskSuggestion> = serde_json::from_str(result).inspect_err(|err| {
            println!("Error parsing subtasks result: {:?}", err);
            println!("raw result: {:?}", result);
        })?;

        Ok(subtasks)
    }

    async fn get_project_suggestion(&self, input: ProjectSuggestionInput) -> Result<ProjectSuggestion, SDKError> {
        let system_message = PlexoSystemTemplate {}.render().unwrap();

        let projects = self
            .get_projects(
                GetProjectsInputBuilder::default()
                    .limit(10)
                    .sort_by("created_at".to_string())
                    .sort_order(SortOrder::Asc)
                    .build()
                    .unwrap(),
            )
            .await?;

        let title = input.title.clone();
        let generate_tasks_number = input.generate_tasks_number.unwrap_or(0);
        let initial_tasks = input.initial_tasks.clone();

        let input_message = ProjectSuggestionTemplate {
            title,
            projects,
            generate_tasks_number,
            initial_tasks,
            initial_state: Some(input),
            user_query: None,
        }
        .render()
        .unwrap();

        let result = self.chat_completion(system_message, input_message).await;
        let result = result.trim().trim_matches('`');

        let suggestion_result: ProjectSuggestion = serde_json::from_str(result).inspect_err(|err| {
            println!("Error parsing project suggestion result: {:?}", err);
            println!("raw result: {:?}", result);
        })?;

        Ok(suggestion_result)
    }

    async fn get_chat_response(
        &self,
        input: ChatResponseInput,
    ) -> Result<Pin<Box<dyn Stream<Item = ChatResponseChunk> + Send>>, SDKError> {
        let chat = self.get_chat(input.chat_id).await?;

        let res_type = chat.resource_type.as_str();

        let system_message = match res_type {
            "project" => {
                let project = self.get_project(chat.resource_id).await?;

                let tasks = self
                    .get_tasks(
                        GetTasksInputBuilder::default()
                            .filter(
                                GetTasksWhereBuilder::default()
                                    .project_id(chat.resource_id)
                                    .build()
                                    .unwrap(),
                            )
                            .sort_by("created_at".to_string())
                            .sort_order(SortOrder::Asc)
                            .limit(10)
                            .build()
                            .ok(),
                    )
                    .await?;

                ProjectRelatedChatTemplate { project, tasks }.render().unwrap()
            }
            "organization" => {
                let organization = self.get_organization().await?.unwrap();

                let projects = self
                    .get_projects(
                        GetProjectsInputBuilder::default()
                            .limit(50)
                            .sort_by("updated_at".to_string())
                            .sort_order(SortOrder::Asc)
                            .build()
                            .unwrap(),
                    )
                    .await?;

                let tasks = self
                    .get_tasks(Some(
                        GetTasksInputBuilder::default()
                            .limit(50)
                            .sort_by("updated_at".to_string())
                            .sort_order(SortOrder::Asc)
                            .build()
                            .unwrap(),
                    ))
                    .await?;

                let members = self
                    .get_members(
                        GetMembersInputBuilder::default()
                            .limit(50)
                            .sort_by("updated_at".to_string())
                            .sort_order(SortOrder::Asc)
                            .build()
                            .unwrap(),
                    )
                    .await?;

                let teams = self
                    .get_teams(
                        GetTeamsInputBuilder::default()
                            .limit(50)
                            .sort_by("updated_at".to_string())
                            .sort_order(SortOrder::Asc)
                            .build()
                            .unwrap(),
                    )
                    .await?;

                OrganizationRelatedChatTemplate {
                    organization,
                    projects,
                    tasks,
                    members,
                    teams,
                }
                .render()
                .unwrap()
            }
            _ => return Err(SDKError::InvalidResourceType),
        };

        let mut messages = self
            .get_messages(
                GetMessagesInputBuilder::default()
                    .sort_by("created_at".to_string())
                    .limit(50)
                    .sort_order(SortOrder::Asc)
                    .filter(GetMessagesWhereBuilder::default().chat_id(chat.id).build().unwrap())
                    .build()
                    .unwrap(),
            )
            .await?;

        messages.push(
            self.create_message(
                CreateMessageInputBuilder::default()
                    .chat_id(chat.id)
                    .owner_id(chat.owner_id)
                    .resource_type(res_type.to_string())
                    .content(
                        serde_json::to_string(&json!({
                            "role": "user",
                            "content": input.message,
                        }))
                        .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .await?,
        );

        let response = self.chat_response(system_message, messages).await;

        let mut total_message = String::new();

        let mut mapped_response = Box::pin(response.map(move |(function_name, delta)| {
            total_message += &delta;

            ChatResponseChunk {
                delta,
                message: total_message.clone(),
                message_id: None,
                tool_call: function_name.map(|a| a.0),
            }
        }));

        let engine = self.clone();
        let res_type = res_type.to_string();

        Ok(Box::pin(stream! {
            let mut last_chunk = None;

            while let Some(chunk) = mapped_response.next().await {
                last_chunk = Some(chunk.clone());
                yield chunk;
            }

            let mut last_chunk_cloned = last_chunk.clone().unwrap();

            let message = engine.create_message(
                CreateMessageInputBuilder::default()
                    .chat_id(chat.id)
                    .owner_id(chat.owner_id)
                    .resource_type(res_type)
                    .content(
                        serde_json::to_string(&json!({
                            "role": "assistant",
                            "content": last_chunk.unwrap().message,
                        }))
                        .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .await
            .unwrap();

            last_chunk_cloned.message_id = Some(message.id);

            yield last_chunk_cloned;
        }))
    }
}
