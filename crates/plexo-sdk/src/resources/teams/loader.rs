use std::{collections::HashMap, str::FromStr, sync::Arc};

use async_graphql::dataloader::Loader;

use uuid::Uuid;

use crate::backend::engine::SDKEngine;

use super::team::{Team, TeamVisibility};

pub struct TeamLoader(Arc<SDKEngine>);

impl TeamLoader {
    pub fn new(e: Arc<SDKEngine>) -> Self {
        Self(e)
    }
}

impl Loader<Uuid> for TeamLoader {
    type Value = Team;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &'_ [Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let teams = sqlx::query!(
            r#"
            SELECT * FROM teams WHERE id  = ANY($1)
            "#,
            &keys
        )
        .fetch_all(&*self.0.db_pool)
        .await
        .unwrap();

        //iterate to get the hashmap
        let teams_map: HashMap<Uuid, Team> = teams
            .iter()
            .map(|team| {
                (
                    team.id,
                    Team {
                        id: team.id,
                        created_at: team.created_at,
                        updated_at: team.updated_at,
                        name: team.name.clone(),
                        owner_id: team.owner_id,
                        visibility: team
                            .visibility
                            .clone()
                            .and_then(|a| TeamVisibility::from_str(&a).ok())
                            .unwrap_or_default(),
                        prefix: team.prefix.clone(),
                    },
                )
            })
            .collect();

        //println!("{:?}", teams);
        Ok(teams_map)
    }
}
