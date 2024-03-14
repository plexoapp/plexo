use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Utc};
use poem_openapi::Enum as OpenApiEnum;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::EnumString;
use uuid::Uuid;

#[derive(Debug, SimpleObject, Object, Clone, Serialize)]
#[graphql(name = "SDKMember")]
pub struct Member {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,
    pub email: String,

    pub role: MemberRole,

    pub github_id: Option<String>,
    pub google_id: Option<String>,

    pub photo_url: Option<String>,

    #[graphql(skip)]
    #[oai(skip)]
    pub password_hash: Option<String>,
}

#[derive(
    Debug, Enum, OpenApiEnum, Copy, Clone, Default, Display, EnumString, Deserialize, Serialize, Eq, PartialEq,
)]
pub enum MemberRole {
    Admin,
    #[default]
    Member,
    ReadOnly,
}
