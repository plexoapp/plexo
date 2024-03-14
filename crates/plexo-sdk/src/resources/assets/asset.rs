use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Utc};

use poem_openapi::Object;
use strum_macros::{Display, EnumString};
use uuid::Uuid;

use poem_openapi::Enum as OpenApiEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, SimpleObject, Object, Clone, Serialize)]
#[graphql(name = "SDKAsset")]
pub struct Asset {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,
    pub kind: AssetKind,
    pub owner_id: Uuid,

    pub project_id: Option<Uuid>,
}

#[derive(
    Debug, Enum, OpenApiEnum, Copy, Clone, Default, Display, EnumString, Deserialize, Serialize, Eq, PartialEq,
)]
pub enum AssetKind {
    #[default]
    Unknown,
    Image,
    Pdf,
    Audio,
    Video,
    Text,
    Website,
}
