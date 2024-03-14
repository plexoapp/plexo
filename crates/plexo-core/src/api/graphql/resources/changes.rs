use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use plexo_sdk::resources::changes::{change::Change as SDKChange, relations::ChangeRelations};

use crate::api::graphql::commons::extract_context;

use super::members::Member;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Change {
    #[graphql(flatten)]
    change: SDKChange,
}

impl From<SDKChange> for Change {
    fn from(val: SDKChange) -> Self {
        Change { change: val }
    }
}

#[ComplexObject]
impl Change {
    async fn owner(&self, ctx: &Context<'_>) -> Result<Member> {
        let (plexo_engine, _change_id) = extract_context(ctx)?;

        self.change
            .owner(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|change| change.into())
    }
}
