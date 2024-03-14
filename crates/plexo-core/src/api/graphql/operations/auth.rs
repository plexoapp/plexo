use async_graphql::{Context, Object, Result, SimpleObject};
use plexo_sdk::resources::members::extensions::{CreateMemberFromEmailInputBuilder, MembersExtensionOperations};

use crate::{api::graphql::commons::extract_context, core::app::Core, errors::app::PlexoAppError};

#[derive(Default)]
pub struct AuthMutation;

#[derive(SimpleObject)]
struct LoginResponse {
    token: String,
    member_id: String,
}

#[Object]
impl AuthMutation {
    async fn login(&self, ctx: &Context<'_>, email: String, password: String) -> Result<LoginResponse> {
        let plexo_engine = ctx.data::<Core>()?.to_owned();

        let Ok(Some(member)) = plexo_engine.engine.get_member_by_email(email.clone()).await else {
            return Err(PlexoAppError::EmailNotFound.into());
        };

        let Some(password_hash) = member.password_hash.clone() else {
            return Err(PlexoAppError::InvalidPassword.into());
        };

        if !plexo_engine.auth.validate_password(password.as_str(), password_hash.as_str()) {
            return Err(PlexoAppError::InvalidPassword.into());
        };

        let Ok(session_token) = plexo_engine.auth.jwt_engine.create_session_token(&member) else {
            return Err(PlexoAppError::InvalidPassword.into());
        };

        Ok(LoginResponse {
            token: session_token,
            member_id: member.id.to_string(),
        })
    }

    async fn register(&self, ctx: &Context<'_>, email: String, name: String, password: String) -> Result<LoginResponse> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        if (plexo_engine.engine.get_member_by_email(email.clone()).await).is_ok_and(|member| member.is_some()) {
            return Err(PlexoAppError::EmailAlreadyExists.into());
        };

        let password_hash = plexo_engine.auth.hash_password(password.as_str());

        let Ok(member) = plexo_engine
            .engine
            .create_member_from_email(
                CreateMemberFromEmailInputBuilder::default()
                    .email(email.clone())
                    .name(name.clone())
                    .password_hash(password_hash)
                    .build()?,
            )
            .await
        else {
            return Err(PlexoAppError::EmailAlreadyExists.into());
        };

        let Ok(session_token) = plexo_engine.auth.jwt_engine.create_session_token(&member) else {
            return Err(PlexoAppError::InvalidPassword.into());
        };

        Ok(LoginResponse {
            token: session_token,
            member_id: member.id.to_string(),
        })
    }
}
