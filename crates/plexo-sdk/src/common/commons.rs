use async_graphql::{Enum, InputObject};
use derive_builder::Builder;
use poem_openapi::{Enum as OpenApiEnum, Object};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use uuid::Uuid;

#[derive(
    Debug, Enum, OpenApiEnum, Copy, Clone, Default, Display, EnumString, Deserialize, Serialize, Eq, PartialEq,
)]
pub enum SortOrder {
    #[default]
    Asc,
    Desc,
}

#[derive(Debug, Clone, Display, EnumString)]
pub enum SQLComparison<T>
where
    T: std::default::Default,
{
    Equal(T),
    NotEqual(T),
    GreaterThan(T),
    GreaterThanOrEqual(T),
    LessThan(T),
    LessThanOrEqual(T),
    Like(T),
    NotLike(T),
    In(T),
    NotIn(T),
    IsNull(T),
    IsNotNull(T),
}

#[derive(Default, Builder, Object, InputObject, Serialize, Clone)]
#[builder(pattern = "owned")]
pub struct UpdateListInput {
    pub add: Vec<Uuid>,
    pub remove: Vec<Uuid>,
}
