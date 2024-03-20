use crate::api::graphql::{
    commons::{create_change, extract_context},
    resources::tasks::Task,
};
use async_graphql::{Context, Object, Result, Subscription};

use plexo_sdk::resources::{
    changes::change::{ChangeOperation, ChangeResourceType, ListenEvent},
    tasks::{
        extensions::{CreateTasksInput, TasksExtensionOperations},
        operations::{CreateTaskInput, GetTasksInput, TaskCrudOperations, UpdateTaskInput},
    },
};
use serde_json::json;
use tokio::task;
use tokio_stream::{Stream, StreamExt};
use uuid::Uuid;

#[derive(Default)]
pub struct TasksGraphQLQuery;

#[Object]
impl TasksGraphQLQuery {
    async fn tasks(&self, ctx: &Context<'_>, input: Option<GetTasksInput>) -> Result<Vec<Task>> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_tasks(input)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|tasks| tasks.into_iter().map(|task| task.into()).collect())
    }

    async fn task(&self, ctx: &Context<'_>, id: Uuid) -> Result<Task> {
        let (core, _member_id) = extract_context(ctx)?;

        core.engine
            .get_task(id)
            .await
            .map_err(|err| async_graphql::Error::new(err.to_string()))
            .map(|task| task.into())
    }
}

#[derive(Default)]
pub struct TasksGraphQLMutation;

#[Object]
impl TasksGraphQLMutation {
    // TODO: It's possible that this method may not work correctly, as the owner_id is being ignored by async_graphql
    async fn create_task(&self, ctx: &Context<'_>, input: CreateTaskInput) -> Result<Task> {
        let (core, member_id) = extract_context(ctx)?;

        let mut input = input;
        input.owner_id = member_id;

        let saved_input = input.clone();

        if let Some( ref mut subtasks) = input.subtasks {
            for subtask in subtasks.iter_mut() {
                subtask.owner_id = input.owner_id;
            }
        };

        let task = core.engine.create_task(input).await?;
        let saved_task = task.clone();

        let input = saved_input.clone();

        task::spawn(async move {
            create_change(
                &core,
                member_id,
                task.id,
                ChangeOperation::Insert,
                ChangeResourceType::Tasks,
                serde_json::to_string(&json!({
                    "input": input,
                    "result": task,
                }))
                .unwrap(),
            )
            .await
            .unwrap();
        });

        Ok(saved_task.into())
    }

    async fn create_tasks(&self, ctx: &Context<'_>, input: CreateTasksInput) -> Result<Vec<Task>> {
        let (core, member_id) = extract_context(ctx)?;

        let mut input = input;
        input.tasks.iter_mut().for_each(|task| task.owner_id = member_id);

        let saved_input = input.clone();

        let tasks = core.engine.create_tasks(input).await?;
        let saved_tasks = tasks.clone();

        tasks.iter().for_each(|task| {
            let core = core.clone();
            let input = saved_input.clone();
            let task = task.clone();

            task::spawn(async move {
                create_change(
                    &core,
                    member_id,
                    task.id,
                    ChangeOperation::Insert,
                    ChangeResourceType::Tasks,
                    serde_json::to_string(&json!({
                        "input": input,
                        "result": task,
                    }))
                    .unwrap(),
                )
                .await
                .unwrap();
            });
        });

        Ok(saved_tasks.into_iter().map(|task| task.into()).collect())
    }

    async fn update_task(&self, ctx: &Context<'_>, id: Uuid, input: UpdateTaskInput) -> Result<Task> {
        let (core, member_id) = extract_context(ctx)?;

        let saved_input = input.clone();

        let task = core.engine.update_task(id, input).await?;

        let task = task.clone();
        let saved_task = task.clone();

        tokio::spawn(async move {
            create_change(
                &core,
                member_id,
                task.id,
                ChangeOperation::Update,
                ChangeResourceType::Tasks,
                serde_json::to_string(&json!({
                    "input": saved_input,
                    "result": task,
                }))
                .unwrap(),
            )
            .await
            .unwrap();
        });

        Ok(saved_task.into())
    }

    async fn delete_task(&self, ctx: &Context<'_>, id: Uuid) -> Result<Task> {
        let (core, _member_id) = extract_context(ctx)?;

        let task = core.engine.delete_task(id).await?;
        let saved_task = task.clone();

        tokio::spawn(async move {
            create_change(
                &core,
                task.owner_id,
                task.id,
                ChangeOperation::Delete,
                ChangeResourceType::Tasks,
                serde_json::to_string(&json!({
                    "result": task,
                }))
                .unwrap(),
            )
            .await
            .unwrap();
        });

        Ok(saved_task.into())
    }
}

#[derive(Default)]
pub struct TasksGraphQLSubscription;

#[Subscription]
impl TasksGraphQLSubscription {
    async fn tasks(&self, ctx: &Context<'_>) -> impl Stream<Item = ListenEvent> {
        let (core, _member_id) = extract_context(ctx).unwrap();

        core.engine
            .listen(ChangeResourceType::Tasks)
            .await
            .unwrap()
            .map(|x| x.unwrap())
    }
}
