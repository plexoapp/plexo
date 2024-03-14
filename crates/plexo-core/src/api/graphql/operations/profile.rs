use async_graphql::{Context, InputObject, Object, Result};
use plexo_sdk::resources::members::{
    extensions::MembersExtensionOperations,
    operations::{MemberCrudOperations, UpdateMemberInputBuilder},
};

use crate::{
    api::graphql::{commons::extract_context, resources::members::Member},
    errors::app::PlexoAppError,
};

#[derive(Default)]
pub struct ProfileGraphQLQuery;

#[Object]
impl ProfileGraphQLQuery {
    async fn me(&self, ctx: &Context<'_>) -> Result<Member> {
        let (core, member_id) = extract_context(ctx)?;

        core.engine
            .get_member(member_id)
            .await
            .map(|member| member.into())
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }
}

#[derive(Default)]
pub struct ProfileGraphQLMutation;

#[derive(InputObject)]
struct UpdateProfileInput {
    name: Option<String>,
    email: Option<String>,
    photo_url: Option<String>,
}

#[derive(InputObject)]
struct UpdatePasswordInput {
    current_password: String,
    new_password: String,
}

#[Object]
impl ProfileGraphQLMutation {
    async fn update_profile(&self, ctx: &Context<'_>, input: UpdateProfileInput) -> Result<Member> {
        let (core, member_id) = extract_context(ctx)?;

        let input_email = input.email;
        let input_name = input.name;
        let input_photo_url = input.photo_url;

        if let Some(email) = input_email.clone() {
            if let Ok(Some(_member)) = core.engine.get_member_by_email(email).await {
                return Err(async_graphql::Error::new("Email already in use"));
            }
        }

        let mut update_member_input = UpdateMemberInputBuilder::default();

        if let Some(email) = input_email {
            update_member_input = update_member_input.email(email);
        }

        if let Some(name) = input_name {
            update_member_input = update_member_input.name(name);
        }

        if let Some(photo_url) = input_photo_url {
            update_member_input = update_member_input.photo_url(photo_url);
        }

        core.engine
            .update_member(member_id, update_member_input.build()?)
            .await
            .map(|member| member.into())
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }

    async fn update_password(&self, ctx: &Context<'_>, input: UpdatePasswordInput) -> Result<Member> {
        let (core, member_id) = extract_context(ctx)?;

        let member = core.engine.get_member(member_id).await?;

        let current_password = input.current_password;
        let new_password = input.new_password;

        let password_hash = member.password_hash.unwrap_or("".to_string());

        if current_password.is_empty() ^ password_hash.is_empty() {
            return Err(PlexoAppError::InvalidPassword.into());
        }

        if !current_password.is_empty() && !core.auth.validate_password(&current_password, &password_hash) {
            return Err(PlexoAppError::InvalidPassword.into());
        }

        let new_password_hash = core.auth.hash_password(&new_password);

        core.engine
            .update_member(
                member_id,
                UpdateMemberInputBuilder::default().password_hash(new_password_hash).build()?,
            )
            .await
            .map(|member| member.into())
            .map_err(|err| async_graphql::Error::new(err.to_string()))
    }
}
