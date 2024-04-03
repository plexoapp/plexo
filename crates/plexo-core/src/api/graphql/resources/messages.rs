use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use plexo_sdk::resources::messages::{message::Message as SDKMessage, relations::MessageRelations};

use crate::api::graphql::commons::extract_context;

use super::members::Member;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Message {
    #[graphql(flatten)]
    message: SDKMessage,
}

impl From<SDKMessage> for Message {
    fn from(val: SDKMessage) -> Self {
        Message { message: val }
    }
}

#[ComplexObject]
impl Message {
    async fn owner(&self, ctx: &Context<'_>) -> Result<Member> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.message
            .owner(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|member| member.into())
    }
}
