use async_graphql::{Context, Result};
use plexo_sdk::resources::changes::{
    change::{Change, ChangeOperation, ChangeResourceType},
    operations::{ChangeCrudOperations, CreateChangeInputBuilder},
};
// use tracing::info;
use uuid::Uuid;

use crate::{auth::resources::PlexoAuthToken, core::app::Core, errors::app::PlexoAppError};

pub fn extract_context(ctx: &Context<'_>) -> Result<(Core, Uuid)> {
    let Ok(auth_token) = &ctx.data::<PlexoAuthToken>() else {
        return Err(PlexoAppError::MissingAuthorizationToken.into());
    };

    let plexo_engine = ctx.data::<Core>()?.to_owned();

    let claims = plexo_engine.auth.extract_claims(auth_token)?;

    let member_id = claims.member_id();

    Ok((plexo_engine, member_id))
}

pub async fn create_change(
    core: &Core,
    owner_id: Uuid,
    resource_id: Uuid,
    operation: ChangeOperation,
    resource_type: ChangeResourceType,
    diff_json: String,
) -> Result<Change> {
    let change = core
        .engine
        .create_change(
            CreateChangeInputBuilder::default()
                .owner_id(owner_id)
                .resource_id(resource_id)
                .operation(operation)
                .resource_type(resource_type)
                .diff_json(diff_json)
                .build()
                .unwrap(),
        )
        .await?;

    // info!("change registered: {:?}", change);

    Ok(change)
}
