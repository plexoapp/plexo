use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use plexo_sdk::resources::chats::{chat::Chat as SDKChat, relations::ChatRelations};

use crate::api::graphql::commons::extract_context;

use super::members::Member;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Chat {
    #[graphql(flatten)]
    chat: SDKChat,
}

impl From<SDKChat> for Chat {
    fn from(val: SDKChat) -> Self {
        Chat { chat: val }
    }
}

#[ComplexObject]
impl Chat {
    async fn owner(&self, ctx: &Context<'_>) -> Result<Member> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.chat
            .owner(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|member| member.into())
    }
}
