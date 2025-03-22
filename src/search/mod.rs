use std::time::Duration;

use anyhow::Result;
use meilisearch_sdk::{client::Client, settings::Settings};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    id: String,
    text: String,
    user_id: String,
}

#[derive(Debug, Clone)]
pub struct Search {
    client: Client,
}

impl Search {
    pub async fn new() -> Result<Self> {
        let client = Client::new("http://127.0.0.1:7700", Some("masterkey"))?;

        let posts_index = client.index("posts");
        posts_index
            .set_settings(
                &Settings::new()
                    .with_searchable_attributes(["text"])
                    .with_filterable_attributes(["user_id"]),
            )
            .await?
            .wait_for_completion(&client, None, Some(Duration::from_secs(60)))
            .await?;

        Ok(Self { client })
    }
}
