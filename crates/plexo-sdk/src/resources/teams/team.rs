use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Utc};

use poem_openapi::Object;
use strum_macros::{Display, EnumString};
use uuid::Uuid;

use poem_openapi::Enum as OpenApiEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, SimpleObject, Object, Clone, Serialize)]
#[graphql(name = "SDKTeam")]
pub struct Team {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,

    pub owner_id: Uuid,

    pub visibility: TeamVisibility,

    pub prefix: Option<String>,
}

#[derive(
    Debug, Enum, OpenApiEnum, Copy, Clone, Default, Display, EnumString, Deserialize, Serialize, Eq, PartialEq,
)]
pub enum TeamVisibility {
    #[default]
    None,
    Public,
    Private,
    Internal,
}
