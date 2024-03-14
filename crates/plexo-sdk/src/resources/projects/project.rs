use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use poem_openapi::Object;
use uuid::Uuid;

use async_graphql::Enum;

use strum_macros::{Display, EnumString};

use poem_openapi::Enum as OpenApiEnum;
use serde::{Deserialize, Serialize};
#[derive(Debug, SimpleObject, Object, Clone, Serialize)]
#[graphql(name = "SDKProject")]
pub struct Project {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,
    pub status: ProjectStatus,
    pub visibility: ProjectVisibility,
    pub owner_id: Uuid,

    pub prefix: Option<String>,
    pub description: Option<String>,

    pub lead_id: Option<Uuid>,
    pub start_date: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(
    Debug, Enum, OpenApiEnum, Copy, Clone, Default, Display, EnumString, Deserialize, Serialize, Eq, PartialEq,
)]

pub enum ProjectStatus {
    #[default]
    None,
    Backlog,
    ToDo,
    InProgress,
    Done,
    Canceled,
}

#[derive(
    Debug, Enum, OpenApiEnum, Copy, Clone, Default, Display, EnumString, Deserialize, Serialize, Eq, PartialEq,
)]
pub enum ProjectVisibility {
    #[default]
    None,
    Private,
    Internal,
    Public,
    // Shared,
}
