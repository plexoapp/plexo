use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use plexo_sdk::resources::labels::{label::Label as SDKLabel, relations::LabelRelations};

use crate::api::graphql::commons::extract_context;

use super::{members::Member, tasks::Task};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Label {
    #[graphql(flatten)]
    label: SDKLabel,
}

impl From<SDKLabel> for Label {
    fn from(val: SDKLabel) -> Self {
        Label { label: val }
    }
}

#[ComplexObject]
impl Label {
    async fn owner(&self, ctx: &Context<'_>) -> Result<Member> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        self.label
            .owner(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|member| member.into())
    }

    async fn tasks(&self, ctx: &Context<'_>) -> Result<Vec<Task>> {
        let (plexo_engine, _label_id) = extract_context(ctx)?;

        self.label
            .tasks(&plexo_engine.loaders)
            .await
            .map_err(|e| e.into())
            .map(|tasks| tasks.into_iter().map(|task| task.into()).collect())
    }
}
