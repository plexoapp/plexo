use std::{error::Error, str::FromStr, sync::Arc};

use dotenv::dotenv;

use plexo_sdk::{
    backend::engine::{SDKConfig, SDKEngine},
    // cognition::{
    // operations::{SubdivideTaskInputBuilder, TaskSuggestionInputBuilder},
    // v2::{operations::CognitionOperationsV2, projects::ProjectSuggestionInputBuilder},
    // },
    // common::commons::SortOrder,
    resources::{
        changes::change::ChangeResourceType,
        projects::operations::{CreateProjectInputBuilder, ProjectCrudOperations},
        tasks::operations::{CreateTaskInputBuilder, TaskCrudOperations}, // tasks::operations::{GetTasksInputBuilder, TaskCrudOperations},
    },
};
use tokio::task;
use tokio_stream::StreamExt;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let engine = SDKEngine::new(SDKConfig::from_env()).await?;
    let engine = Arc::new(engine);

    println!("version: {:?}", engine.version()?);

    let mut tasks_listener = engine.listen(ChangeResourceType::Tasks).await.unwrap();
    let mut projects_listener = engine.listen(ChangeResourceType::Projects).await.unwrap();

    task::spawn(async move {
        while let Some(Ok(notification)) = tasks_listener.next().await {
            println!("task event: {:?}", notification);
        }
    });

    task::spawn(async move {
        while let Some(Ok(notification)) = projects_listener.next().await {
            println!("project event: {:?}", notification);
        }
    });

    engine
        .create_task(
            CreateTaskInputBuilder::default()
                .owner_id(Uuid::from_str("f068e8e6-249e-41be-b8aa-bdc84c8a0444")?)
                .title("task 0x07".to_string())
                .build()?,
        )
        .await?;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    engine
        .create_task(
            CreateTaskInputBuilder::default()
                .owner_id(Uuid::from_str("f068e8e6-249e-41be-b8aa-bdc84c8a0444")?)
                .title("task 0x07".to_string())
                .build()?,
        )
        .await?;

    let engine_2 = engine.clone();

    task::spawn(async move {
        engine
            .create_task(
                CreateTaskInputBuilder::default()
                    .owner_id(Uuid::from_str("f068e8e6-249e-41be-b8aa-bdc84c8a0444").unwrap())
                    .title("task 0x07".to_string())
                    .build()
                    .unwrap(),
            )
            .await
            .unwrap();

        // println!("task: {:?}", task.id);
    });

    task::spawn(async move {
        engine_2
            .create_project(
                CreateProjectInputBuilder::default()
                    .owner_id(Uuid::from_str("f068e8e6-249e-41be-b8aa-bdc84c8a0444").unwrap())
                    .name("p0001".to_string())
                    .build()
                    .unwrap(),
            )
            .await
            .unwrap();
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    Ok(())
}
