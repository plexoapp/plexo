use std::{collections::HashMap, sync::Arc};

use async_graphql::dataloader::Loader;

use uuid::Uuid;

use crate::backend::engine::SDKEngine;

use super::label::Label;

// #[derive(Clone)]
pub struct LabelLoader(Arc<SDKEngine>);

impl LabelLoader {
    pub fn new(e: Arc<SDKEngine>) -> Self {
        Self(e)
    }
}

impl Loader<Uuid> for LabelLoader {
    type Value = Label;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &'_ [Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let labels = sqlx::query!(
            r#"
            SELECT * FROM labels WHERE id  = ANY($1)
            "#,
            &keys
        )
        .fetch_all(&*self.0.db_pool)
        .await
        .unwrap();

        //iterate to get the hashmap
        let labels_map: HashMap<Uuid, Label> = labels
            .iter()
            .map(|label| {
                (
                    label.id,
                    Label {
                        id: label.id,
                        created_at: label.created_at,
                        updated_at: label.updated_at,
                        name: label.name.clone(),
                        owner_id: label.owner_id,
                        description: label.description.clone(),
                        color: label.color.clone(),
                    },
                )
            })
            .collect();

        //println!("{:?}", labels);
        Ok(labels_map)
    }
}
