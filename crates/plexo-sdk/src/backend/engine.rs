use std::{env::var, pin::Pin, str::FromStr, time::Duration};

use async_openai::{config::OpenAIConfig, Client};
use sqlx::{
    postgres::{PgListener, PgPoolOptions},
    Pool, Postgres,
};

use tokio_stream::{Stream, StreamExt};
use uuid::Uuid;
// use tokio::runtime::Handle;

use crate::{
    errors::sdk::SDKError,
    organization::operations::{
        Organization, OrganizationCrudOperations, OrganizationInitializationInput, SetOrganizationInputBuilder,
        GLOBAL_ORGANIZATION_SETTINGS_NAME,
    },
    resources::changes::change::{ChangeOperation, ChangeResourceType, ListenEvent}, // resources::tasks::task::Task,
};
// use crossbeam_channel::unbounded;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[derive(Clone)]
pub struct SDKConfig {
    pub database_url: String,
    pub llm_api_key: String,
    pub llm_model_name: String,
}

impl SDKConfig {
    pub fn from_env() -> SDKConfig {
        let database_url = var("DATABASE_URL").unwrap();
        let llm_api_key = var("OPENAI_API_KEY").unwrap();
        let llm_model_name = var("OPENAI_MODEL_NAME").unwrap_or("gpt-3.5-turbo".to_string());

        SDKConfig {
            database_url,
            llm_api_key,
            llm_model_name,
        }
    }
}

#[derive(Clone)]
pub struct SDKEngine {
    pub config: SDKConfig,
    pub db_pool: Box<Pool<Postgres>>,
    // pub db_listener: PgListener,
    pub llm_client: Box<Client<OpenAIConfig>>,
    // pub task_event_send: crossbeam_channel::Sender<Task>,
    // pub task_event_recv: crossbeam_channel::Receiver<Task>,
}

impl SDKEngine {
    pub async fn new(config: SDKConfig) -> Result<SDKEngine, SDKError> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(60))
            .connect(config.database_url.as_str())
            .await?;

        let llm_config = OpenAIConfig::default().with_api_key(config.llm_api_key.clone());

        let llm_client = Box::new(Client::with_config(llm_config));

        let db_pool = Box::new(pool);

        // let (task_event_send, task_event_recv) = unbounded::<Task>();
        // let listener = PgListener::connect(&config.database_url).await?;

        // let db_listener = PgListener::connect_with(&db_pool).await?;

        // task::spawn(async move {
        //     while let Some(notification) = db_listener.try_recv().await.unwrap() {
        //         println!("notification: {:?}", notification);
        //     }
        //     // while let Ok(notification) = listener.recv().await {
        //     //     println!("notification: {:?}", notification);
        //     // }
        // });

        // let a = db_listener.into_stream().;

        let engine = SDKEngine {
            config,
            db_pool,
            llm_client,
            // db_listener,
            // task_event_send,
            // task_event_recv,
        };

        Ok(engine)
    }

    pub async fn migrate(&self) -> Result<(), SDKError> {
        sqlx::migrate!().run(self.db_pool.as_ref()).await?;

        Ok(())
    }

    pub fn version(&self) -> Result<String, SDKError> {
        match VERSION {
            Some(version) => Ok(version.to_string()),
            None => Err(SDKError::VersionNotFound),
        }
    }

    pub async fn initialize_organization(
        &self,
        owner_id: Uuid,
        value: OrganizationInitializationInput,
    ) -> Result<Organization, SDKError> {
        let org_serialized = serde_json::to_string(&value)?;

        let org = self
            .set_organization_setting(
                SetOrganizationInputBuilder::default()
                    .owner_id(owner_id)
                    .name(GLOBAL_ORGANIZATION_SETTINGS_NAME.to_string())
                    .value(org_serialized)
                    .build()
                    .unwrap(),
            )
            .await?;

        Ok(org.into())
    }

    pub async fn listen(
        &self,
        resource: ChangeResourceType,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ListenEvent, SDKError>> + Send>>, SDKError> {
        let mut db_listener = PgListener::connect_with(&self.db_pool).await?;

        db_listener
            .listen(format!("{}_table_update", resource.to_string().to_lowercase()).as_str())
            .await?;

        let mapped_stream = db_listener.into_stream().map(|x| match x {
            Ok(not) => {
                // TG_TABLE_NAME || ' ' || TG_OP || ' ' || row.id;

                let mut payload = not.payload().split_whitespace();

                let resource = ChangeResourceType::from_str(payload.next().unwrap()).unwrap();
                let operation = ChangeOperation::from_str(payload.next().unwrap()).unwrap();
                let row_id = payload.next().map(|a| a.parse::<Uuid>().unwrap()).unwrap();

                Ok(ListenEvent {
                    resource,
                    operation,
                    row_id,
                })
            }
            Err(e) => Err(SDKError::from(e)),
        });

        Ok(Box::pin(mapped_stream))
    }
}
