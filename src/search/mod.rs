use std::env;
use std::time::Duration;

use anyhow::{Context, Result};
use meilisearch_sdk::{client::Client, settings::Settings};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: String,
    pub text: String,
    pub user_id: String,
}

#[derive(Debug, Clone)]
pub struct Search {
    client: Client,
}

impl Search {
    pub async fn new() -> Result<Self> {
        let meili_url =
            env::var("MEILISEARCH_URL").unwrap_or_else(|_| "http://127.0.0.1:7700".to_string());
        let client = Client::new(&meili_url, Some("masterkey"))?;

        let posts_index = client.index("posts");
        posts_index
            .set_settings(
                &Settings::new()
                    .with_searchable_attributes(["text"])
                    .with_filterable_attributes(["user_id"]),
            )
            .await
            .context("Failed to connect to Meilisearch")?
            .wait_for_completion(&client, None, Some(Duration::from_secs(60)))
            .await?;

        Ok(Self { client })
    }

    /// Insert or update a post in the search index
    pub async fn post_upsert(&self, post: Post) -> Result<()> {
        let posts_index = self.client.index("posts");
        posts_index
            .add_documents(&[post], Some("id"))
            .await?
            .wait_for_completion(&self.client, None, Some(Duration::from_secs(60)))
            .await?;
        Ok(())
    }

    /// Delete a post from the search index by ID
    pub async fn post_delete(&self, post_id: &str) -> Result<()> {
        let posts_index = self.client.index("posts");
        posts_index
            .delete_document(post_id)
            .await?
            .wait_for_completion(&self.client, None, Some(Duration::from_secs(60)))
            .await?;
        Ok(())
    }

    /// Search for posts with optional user filtering
    ///
    /// * `query` - The search text to query for
    /// * `user_id` - Optional user ID to filter results by
    pub async fn post_search(&self, query: &str, user_id: Option<&str>) -> Result<Vec<Post>> {
        let posts_index = self.client.index("posts");
        let mut search_query = posts_index.search();

        // Set the query text
        search_query.with_query(query);

        let filter = user_id.map(|uid| format!("user_id = \"{}\"", uid));
        if let Some(filter) = filter.as_ref() {
            search_query.with_filter(filter);
        }

        let search_results = search_query.execute().await?;

        Ok(search_results
            .hits
            .into_iter()
            .map(|hit| hit.result)
            .collect())
    }
}
