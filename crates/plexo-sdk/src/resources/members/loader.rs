use std::{collections::HashMap, str::FromStr, sync::Arc};

use async_graphql::dataloader::Loader;

use uuid::Uuid;

use crate::backend::engine::SDKEngine;

use super::member::{Member, MemberRole};

// #[derive(Clone)]
pub struct MemberLoader(Arc<SDKEngine>);

impl MemberLoader {
    pub fn new(e: Arc<SDKEngine>) -> Self {
        Self(e)
    }
}

impl Loader<Uuid> for MemberLoader {
    type Value = Member;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &'_ [Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let members = sqlx::query!(
            r#"
            SELECT * FROM members WHERE id  = ANY($1)
            "#,
            &keys
        )
        .fetch_all(&*self.0.db_pool)
        .await
        .unwrap();

        //iterate to get the hashmap
        let members_map: HashMap<Uuid, Member> = members
            .iter()
            .map(|member| {
                (
                    member.id,
                    Member {
                        id: member.id,
                        created_at: member.created_at,
                        updated_at: member.updated_at,
                        name: member.name.clone(),
                        email: member.email.clone(),
                        role: member
                            .role
                            .clone()
                            .and_then(|a| MemberRole::from_str(&a).ok())
                            .unwrap_or_default(),
                        github_id: member.github_id.clone(),
                        google_id: member.google_id.clone(),
                        photo_url: member.photo_url.clone(),
                        password_hash: member.password_hash.clone(),
                    },
                )
            })
            .collect();

        //println!("{:?}", members);
        Ok(members_map)
    }
}
