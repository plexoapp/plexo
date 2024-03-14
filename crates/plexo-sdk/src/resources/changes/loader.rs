use std::{collections::HashMap, str::FromStr, sync::Arc};

use async_graphql::dataloader::Loader;

use uuid::Uuid;

use crate::backend::engine::SDKEngine;

use super::change::{Change, ChangeOperation, ChangeResourceType};

// #[derive(Clone)]
pub struct ChangeLoader(Arc<SDKEngine>);

impl ChangeLoader {
    pub fn new(e: Arc<SDKEngine>) -> Self {
        Self(e)
    }
}

impl Loader<Uuid> for ChangeLoader {
    type Value = Change;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &'_ [Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let changes = sqlx::query!(
            r#"
            SELECT * FROM changes WHERE id  = ANY($1)
            "#,
            &keys
        )
        .fetch_all(&*self.0.db_pool)
        .await
        .unwrap();

        //iterate to get the hashmap
        let changes_map: HashMap<Uuid, Change> = changes
            .iter()
            .map(|change| {
                (
                    change.id,
                    Change {
                        id: change.id,
                        created_at: change.created_at,
                        updated_at: change.updated_at,
                        owner_id: change.owner_id,
                        resource_id: change.resource_id,
                        operation: ChangeOperation::from_str(change.operation.as_str()).unwrap(),
                        resource_type: ChangeResourceType::from_str(change.resource_type.as_str()).unwrap(),
                        diff_json: change.diff_json.clone(),
                    },
                )
            })
            .collect();

        //println!("{:?}", changes);
        Ok(changes_map)
    }
}
