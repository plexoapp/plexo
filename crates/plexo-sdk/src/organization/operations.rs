use async_graphql::{InputObject, SimpleObject};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::{backend::engine::SDKEngine, errors::sdk::SDKError};

pub const GLOBAL_ORGANIZATION_SETTINGS_NAME: &str = "principal";

#[derive(Debug, SimpleObject, Object, Clone, Serialize)]
#[graphql(name = "SDKOrganization")]
pub struct Organization {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub owner_id: Uuid,
    pub name: String,
    pub raw_value: String,

    pub photo_url: String,
    pub email: String,
    pub description: Option<String>,
    pub hub_id: Option<String>,
    pub plan_id: Option<String>,
}

#[derive(Debug, SimpleObject, Object, Clone, Serialize)]
#[graphql(name = "SDKOrganizationSettings")]
pub struct OrganizationSettings {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub owner_id: Uuid,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
#[builder(pattern = "owned")]
pub struct OrganizationInitializationInput {
    pub owner_id: Uuid,
    pub name: String,
    pub photo_url: String,
    pub email: String,

    #[builder(setter(strip_option), default)]
    pub description: Option<String>,
    #[builder(setter(strip_option), default)]
    pub hub_id: Option<String>,
    #[builder(setter(strip_option), default)]
    pub plan_id: Option<String>,
}

#[derive(Default, Builder, Object, InputObject, Serialize, Clone)]
#[builder(pattern = "owned")]
pub struct UpdateOrganizationInput {
    #[builder(setter(strip_option), default)]
    pub name: Option<String>,
    #[builder(setter(strip_option), default)]
    pub photo_url: Option<String>,
    #[builder(setter(strip_option), default)]
    pub email: Option<String>,
    #[builder(setter(strip_option), default)]
    pub description: Option<String>,

    #[builder(setter(strip_option), default)]
    pub hub_id: Option<String>,
    #[builder(setter(strip_option), default)]
    pub plan_id: Option<String>,
}

#[derive(Default, Builder, Object, InputObject, Serialize, Clone)]
#[builder(pattern = "owned")]
pub struct SetOrganizationInput {
    pub owner_id: Uuid,
    pub name: String,
    pub value: String,
}

impl From<OrganizationSettings> for Organization {
    fn from(org_setting: OrganizationSettings) -> Self {
        let org: Value = serde_json::from_str(&org_setting.value).unwrap();

        Organization {
            id: org_setting.id,
            created_at: org_setting.created_at,
            updated_at: org_setting.updated_at,
            owner_id: org_setting.owner_id,
            raw_value: org_setting.value,

            name: org["name"].as_str().unwrap().to_string(),
            photo_url: org["photo_url"].as_str().unwrap().to_string(),
            email: org["email"].as_str().unwrap().to_string(),
            description: org["description"].as_str().map(|s| s.to_string()),
            hub_id: org["hub_id"].as_str().map(|s| s.to_string()),
            plan_id: org["plan_id"].as_str().map(|s| s.to_string()),
        }
    }
}

#[async_trait]
pub trait OrganizationCrudOperations {
    async fn get_organization(&self) -> Result<Option<Organization>, SDKError>;
    async fn update_organization(&self, input: UpdateOrganizationInput) -> Result<Organization, SDKError>;

    async fn get_organization_settings(&self) -> Result<Vec<OrganizationSettings>, SDKError>;

    async fn set_organization_setting(&self, input: SetOrganizationInput) -> Result<OrganizationSettings, SDKError>;
    async fn get_organization_setting(&self, name: String) -> Result<Option<OrganizationSettings>, SDKError>;
}

#[async_trait]
impl OrganizationCrudOperations for SDKEngine {
    async fn get_organization(&self) -> Result<Option<Organization>, SDKError> {
        let principal_settings = self
            .get_organization_setting(GLOBAL_ORGANIZATION_SETTINGS_NAME.to_string())
            .await?;

        Ok(principal_settings.map(|org| org.into()))
    }

    async fn update_organization(&self, _input: UpdateOrganizationInput) -> Result<Organization, SDKError> {
        todo!()
    }

    async fn get_organization_settings(&self) -> Result<Vec<OrganizationSettings>, SDKError> {
        let task_info = sqlx::query!(
            r#"
            SELECT * FROM organization
            "#,
        )
        .fetch_all(self.db_pool.as_ref())
        .await?;

        Ok(task_info
            .into_iter()
            .map(|task_info| OrganizationSettings {
                id: task_info.id,
                created_at: task_info.created_at,
                updated_at: task_info.updated_at,
                owner_id: task_info.owner_id,
                name: task_info.name,
                value: task_info.value,
            })
            .collect())
    }

    async fn set_organization_setting(&self, input: SetOrganizationInput) -> Result<OrganizationSettings, SDKError> {
        let task_info = sqlx::query!(
            r#"
            INSERT INTO organization (name, value, owner_id)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            input.name,
            input.value,
            input.owner_id
        )
        .fetch_one(self.db_pool.as_ref())
        .await?;

        Ok(OrganizationSettings {
            id: task_info.id,
            created_at: task_info.created_at,
            updated_at: task_info.updated_at,
            owner_id: task_info.owner_id,
            name: task_info.name,
            value: task_info.value,
        })
    }

    async fn get_organization_setting(&self, name: String) -> Result<Option<OrganizationSettings>, SDKError> {
        let task_info = sqlx::query!(
            r#"
            SELECT * FROM organization WHERE name = $1
            "#,
            name,
        )
        .fetch_one(self.db_pool.as_ref())
        .await;

        match task_info {
            Err(sqlx::Error::RowNotFound) => return Ok(None),
            Err(err) => return Err(err.into()),
            Ok(task_info) => Ok(Some(OrganizationSettings {
                id: task_info.id,
                created_at: task_info.created_at,
                updated_at: task_info.updated_at,
                owner_id: task_info.owner_id,
                name: task_info.name,
                value: task_info.value,
            })),
        }
    }
}
