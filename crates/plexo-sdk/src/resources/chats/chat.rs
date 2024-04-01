use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Utc};
use poem_openapi::Enum as OpenApiEnum;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use uuid::Uuid;

#[derive(Debug, SimpleObject, Object, Clone, Serialize)]
#[graphql(name = "SDKChat")]
pub struct Chat {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub owner_id: Uuid,
    pub resource_id: Uuid,
    pub resource_type: String,
    pub status: ChatStatus,
}

#[derive(
    Debug, Enum, OpenApiEnum, Copy, Clone, Default, Display, EnumString, Deserialize, Serialize, Eq, PartialEq,
)]
pub enum ChatStatus {
    #[default]
    None,
    Active,
    Archived,
}
