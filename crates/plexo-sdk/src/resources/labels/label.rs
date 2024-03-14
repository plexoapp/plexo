use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};

use poem_openapi::Object;

use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, SimpleObject, Object, Clone, Serialize)]
#[graphql(name = "SDKLabel")]
pub struct Label {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,
    pub owner_id: Uuid,

    pub description: Option<String>,
    pub color: Option<String>,
}
