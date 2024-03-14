use std::str::FromStr;

use async_graphql::InputObject;
use async_trait::async_trait;
use derive_builder::Builder;

use crate::{backend::engine::SDKEngine, errors::sdk::SDKError};

use super::member::{Member, MemberRole};

#[async_trait]
pub trait MembersExtensionOperations {
    async fn create_member_from_github(&self, input: CreateMemberFromGithubInput) -> Result<Member, SDKError>;
    async fn create_member_from_email(&self, input: CreateMemberFromEmailInput) -> Result<Member, SDKError>;
    async fn get_member_by_github_id(&self, github_id: String) -> Result<Option<Member>, SDKError>;
    async fn get_member_by_email(&self, email: String) -> Result<Option<Member>, SDKError>;
}

#[derive(Default, Builder, InputObject)]
#[builder(pattern = "owned")]
pub struct CreateMemberFromGithubInput {
    github_id: String,
    name: String,
    email: String,
    photo_url: Option<String>,
}

#[derive(Default, Builder, InputObject)]
#[builder(pattern = "owned")]
pub struct CreateMemberFromEmailInput {
    email: String,
    name: String,
    password_hash: String,
    #[builder(setter(strip_option), default)]
    role: Option<MemberRole>,
    #[builder(setter(strip_option), default)]
    photo_url: Option<String>,
}

#[async_trait]
impl MembersExtensionOperations for SDKEngine {
    async fn create_member_from_github(&self, input: CreateMemberFromGithubInput) -> Result<Member, SDKError> {
        let member_info = sqlx::query!(
            "
            INSERT INTO members (email, name, github_id, photo_url)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            ",
            input.email,
            input.name,
            input.github_id,
            input.photo_url,
        )
        .fetch_one(&*self.db_pool)
        .await?;

        Ok(Member {
            id: member_info.id,
            email: member_info.email,
            name: member_info.name,
            created_at: member_info.created_at,
            updated_at: member_info.updated_at,
            github_id: member_info.github_id,
            google_id: member_info.google_id,
            photo_url: member_info.photo_url,
            role: member_info
                .role
                .and_then(|a| MemberRole::from_str(&a).ok())
                .unwrap_or_default(),
            password_hash: member_info.password_hash,
        })
    }

    async fn create_member_from_email(&self, input: CreateMemberFromEmailInput) -> Result<Member, SDKError> {
        let member_info = sqlx::query!(
            "
            INSERT INTO members (email, name, password_hash, photo_url, role)
            VALUES ($1, $2, $3, $4, COALESCE($5))
            RETURNING *
            ",
            input.email,
            input.name,
            input.password_hash,
            input.photo_url,
            input.role.map(|role| role.to_string()),
        )
        .fetch_one(&*self.db_pool)
        .await?;

        Ok(Member {
            id: member_info.id,
            email: member_info.email,
            name: member_info.name,
            created_at: member_info.created_at,
            updated_at: member_info.updated_at,
            github_id: member_info.github_id,
            google_id: member_info.google_id,
            photo_url: member_info.photo_url,
            role: member_info
                .role
                .and_then(|a| MemberRole::from_str(&a).ok())
                .unwrap_or_default(),
            password_hash: member_info.password_hash,
        })
    }

    async fn get_member_by_github_id(&self, github_id: String) -> Result<Option<Member>, SDKError> {
        let member_info = sqlx::query!(
            "
            SELECT * FROM members
            WHERE github_id = $1
            ",
            github_id,
        )
        .fetch_one(&*self.db_pool)
        .await?;

        Ok(Some(Member {
            id: member_info.id,
            email: member_info.email,
            name: member_info.name,
            created_at: member_info.created_at,
            updated_at: member_info.updated_at,
            github_id: member_info.github_id,
            google_id: member_info.google_id,
            photo_url: member_info.photo_url,
            role: member_info
                .role
                .and_then(|a| MemberRole::from_str(&a).ok())
                .unwrap_or_default(),
            password_hash: member_info.password_hash,
        }))
    }

    async fn get_member_by_email(&self, email: String) -> Result<Option<Member>, SDKError> {
        let member_info = sqlx::query!(
            "
            SELECT * FROM members
            WHERE email = $1
            ",
            email,
        )
        .fetch_one(&*self.db_pool)
        .await;

        match member_info {
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(SDKError::from(e)),
            Ok(member_info) => Ok(Some(Member {
                id: member_info.id,
                email: member_info.email,
                name: member_info.name,
                created_at: member_info.created_at,
                updated_at: member_info.updated_at,
                github_id: member_info.github_id,
                google_id: member_info.google_id,
                photo_url: member_info.photo_url,
                role: member_info
                    .role
                    .and_then(|a| MemberRole::from_str(&a).ok())
                    .unwrap_or_default(),
                password_hash: member_info.password_hash,
            })),
        }
    }
}
