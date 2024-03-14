use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Utc};

use poem_openapi::Object;
use strum_macros::{Display, EnumString};
use uuid::Uuid;

use poem_openapi::Enum as OpenApiEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, SimpleObject, Object, Clone, Serialize)]
#[graphql(name = "SDKTask")]
pub struct Task {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub title: String,
    pub owner_id: Uuid,
    pub status: TaskStatus,
    pub priority: TaskPriority,

    pub count: i32,

    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub project_id: Option<Uuid>,
    pub lead_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
}

#[derive(
    Debug, Enum, OpenApiEnum, Copy, Clone, Default, Display, EnumString, Deserialize, Serialize, Eq, PartialEq,
)]

pub enum TaskStatus {
    #[default]
    None,
    Draft,
    Backlog,
    ToDo,
    InProgress,
    Done,
    Canceled,
}

#[derive(
    Debug, Enum, OpenApiEnum, Copy, Clone, Default, Display, EnumString, Deserialize, Serialize, Eq, PartialEq,
)]

pub enum TaskPriority {
    #[default]
    None,
    Low,
    Medium,
    High,
    Urgent,
}
