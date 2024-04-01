use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Utc};
use poem_openapi::Enum as OpenApiEnum;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use uuid::Uuid;

#[derive(Debug, SimpleObject, Object, Clone, Serialize, Deserialize)]
#[graphql(name = "SDKMessage")]
pub struct Message {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub owner_id: Uuid,
    pub chat_id: Uuid,
    pub content: String,
    pub status: MessageStatus,
    pub parent_id: Option<Uuid>,
}

#[derive(
    Debug, Enum, OpenApiEnum, Copy, Clone, Default, Display, EnumString, Deserialize, Serialize, Eq, PartialEq,
)]
pub enum MessageStatus {
    #[default]
    Sent,
    Received,
    Read,
}
