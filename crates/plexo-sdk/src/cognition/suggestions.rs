use super::{
    operations::TaskSuggestionInput,
    v2::chat::{ChatResponseFunctionCall, ChatResponseToolCall},
};

use crate::{
    backend::engine::SDKEngine,
    resources::{
        messages::message::Message,
        tasks::{
            operations::{GetTasksInputBuilder, TaskCrudOperations},
            task::{Task, TaskPriority, TaskStatus},
        },
    },
};

use async_openai::types::{
    ChatChoiceStream, ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, ChatCompletionStreamResponseDelta, ChatCompletionToolArgs,
    ChatCompletionToolType, CreateChatCompletionRequestArgs, FunctionObjectArgs,
};

use async_stream::stream;
use async_trait::async_trait;
use schemars::{schema_for, JsonSchema};

use serde_json::{json, Value};
use std::{collections::HashMap, pin::Pin, sync::Arc};
use tokio_stream::{Stream, StreamExt};
use uuid::Uuid;

use tokio::sync::Mutex;
#[async_trait]
pub trait CognitionCapabilities {
    async fn chat_completion(&self, system_message: String, user_message: String) -> String;
    async fn acquire_tasks_fingerprints(&self, number_of_tasks: u32, project_id: Option<Uuid>) -> Vec<String>;
    async fn chat_response(
        &self,
        system_message: String,
        messages: Vec<Message>,
    ) -> Pin<Box<dyn Stream<Item = (Option<Vec<ChatResponseToolCall>>, String)> + Send>>;

    fn calculate_task_fingerprint(task: Task) -> String;
    fn calculate_task_suggestion_fingerprint(task_suggestion: TaskSuggestionInput) -> String;
    fn message_to_chat_completion(message: &Message) -> ChatCompletionRequestMessage;
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

    async fn chat_response(
        &self,
        system_message: String,
        messages: Vec<Message>,
    ) -> Pin<Box<dyn Stream<Item = (Option<Vec<ChatResponseToolCall>>, String)> + Send>> {
        let mut conversation_messages: Vec<ChatCompletionRequestMessage> =
            messages.iter().map(Self::message_to_chat_completion).collect();

        let mut messages: Vec<ChatCompletionRequestMessage> = vec![ChatCompletionRequestSystemMessageArgs::default()
            .content(system_message)
            .build()
            .unwrap()
            .into()];

        messages.append(&mut conversation_messages);

        let create_many_task_input_schema = schema_for!(Vec<CreateTaskLLMFunctionInput>);

        let create_many_task_function_def = json!({
            "type": "object",
            "properties": {
                "input": &create_many_task_input_schema,
            },
            "required": ["input"],
        });

        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(1024u16)
            .model(self.config.llm_model_name.clone())
            .messages(messages)
            .tools(vec![ChatCompletionToolArgs::default()
                .r#type(ChatCompletionToolType::Function)
                .function(
                    FunctionObjectArgs::default()
                        .name("create_many_tasks")
                        .description(
                            "Create a list of tasks in the database. The function takes a list of CreateTaskLLMFunctionInput as input and returns a list of Task created.".to_string(),
                        )
                        .parameters(create_many_task_function_def)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap()])
            .build()
            .unwrap();

        // println!("request: {}", serde_json::to_string(&request).unwrap());

        let mut response = self.llm_client.chat().create_stream(request).await.unwrap();

        // let mut tool_name = None;

        let tool_calls = Arc::new(Mutex::new(HashMap::<String, ChatResponseToolCall>::new()));

        // let tool_call_states: Arc<Mutex<HashMap<(i32, i32), ChatCompletionMessageToolCall>>> = Arc::new(Mutex::new(HashMap::new()));

        // tool_calls.en
        Box::pin(stream! {
            while let Some(response) = response.next().await {
                match response.unwrap().choices.first().unwrap() { // TODO: change this .first() to accept many choices
                    ChatChoiceStream{delta: ChatCompletionStreamResponseDelta {content: Some(content), ..}, ..} => {
                        yield (None, content.clone());
                    }

                    ChatChoiceStream{delta: ChatCompletionStreamResponseDelta {tool_calls: Some(calls), ..}, ..} => {
                        for (index, call) in calls.iter().enumerate() {
                            // println!("index: {}", index);
                            // let tool = call.function.clone().unwrap();

                            // if let Some(name) = tool.clone().name {
                            //    let  _tool_name = Some(name);
                            // }

                            let mut tool_calls = tool_calls.lock().await;

                            // let mut states_lock = states.lock().await;


                            let state = tool_calls.entry(format!("{}", index)).or_insert_with(|| {
                                ChatResponseToolCall {
                                    id: call.id.clone(),
                                    // r#type: call.r#type.clone().map(|t| serde_json::to_string(&t).unwrap()),
                                    // r#type: Some("function".to_string()), // TODO: Improve this
                                    r#type: call.r#type.clone().map(|t| serde_json::to_value(t).unwrap_or(Value::String("function".to_string())).as_str().unwrap().to_string()),
                                    function: Some(ChatResponseFunctionCall {
                                        name: call.function.clone().unwrap().name.clone(),
                                        arguments: call.function.clone().unwrap().arguments.clone(),
                                    }),
                                }
                            });

                            if let Some(arguments) = call
                                .function
                                .as_ref()
                                .and_then(|f| f.arguments.as_ref())
                            {
                                state.function.as_mut().unwrap().arguments.as_mut().unwrap().push_str(arguments);

                                yield (Some(Vec::from_iter(tool_calls.values()).into_iter().cloned().collect()), arguments.clone());
                            }

                            // if tool_calls.contains_key(call.id.clone().unwrap().as_str()) {
                            //     let value_in_tool_calls = tool_calls.get_mut(call.id.clone().unwrap().as_str()).unwrap();

                            //     value_in_tool_calls.id;

                            //     // value_in_tool_calls.function.as_mut().and_then(|f| {
                            //     //     if f.arguments.is_none() {
                            //     //         value_in_tool_calls.function = Some(ChatResponseFunctionCall {
                            //     //             name: tool.name.clone(),
                            //     //             arguments: tool.arguments.clone(),
                            //     //         });
                            //     //     }

                            //     //     Some(())
                            //     // });

                            //     continue;
                            // }

                            // let tool_arguments = tool.arguments.clone().unwrap();

                            // tool_calls.insert(call.id.clone().unwrap(), ChatResponseToolCall {
                            //     id: call.id.clone(),
                            //     function: Some(ChatResponseFunctionCall {
                            //         name: tool.name.clone(),
                            //         arguments: tool.arguments.clone(),
                            //     }),
                            // });


                        }
                    }
                    ChatChoiceStream{finish_reason: Some(_finish_reason), ..} => {
                        // println!("finish_reason: {:?}", finish_reason);
                        break;
                    },

                    choice => {
                        println!("choice: {:?}", choice);
                        continue;
                        // yield (None, "<UNK>".to_string());
                    }
                }
            }
        })
    }

    fn message_to_chat_completion(message: &'_ Message) -> ChatCompletionRequestMessage {
        let val: Value = serde_json::from_str(message.content.as_str()).unwrap();

        match val.clone() {
            Value::Object(obj) => match obj.get("role").unwrap().as_str().unwrap() {
                "user" => ChatCompletionRequestMessage::User(serde_json::from_value(val).unwrap()),
                "assistant" => ChatCompletionRequestMessage::Assistant(serde_json::from_value(val).unwrap()),
                "tool" => ChatCompletionRequestMessage::Tool(serde_json::from_value(val).unwrap()),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

// pub struct ToolExecution(pub String);

#[derive(Clone, Default, JsonSchema)]
pub struct CreateTaskLLMFunctionInput {
    pub title: String,

    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub description: Option<String>,
    pub due_date: Option<String>,
    pub project_id: Option<String>,
    pub lead_id: Option<String>,
    pub parent_id: Option<String>,
    pub labels: Option<Vec<String>>,
    pub assignees: Option<Vec<String>>,
    // pub subtasks: Option<Vec<CreateTaskInput>>,
    // pub assets: Option<Vec<String>>,
}
