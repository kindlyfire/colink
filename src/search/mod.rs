use std::time::Duration;

use anyhow::{Context, Result};
use jiff::Timestamp;
use meilisearch_sdk::{client::Client, settings::Settings as MeilisearchSettings};
use serde::{Deserialize, Serialize};

use crate::{db::models::posts, settings::Settings};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: String,
    pub text: String,
    pub user_id: String,
    pub created_at: i64,
}

impl TryFrom<&posts::Model> for Post {
    type Error = anyhow::Error;
    fn try_from(post: &posts::Model) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            id: post.id.clone(),
            text: post.text.clone(),
            user_id: post.user_id.clone(),
            created_at: post.created_at.parse::<Timestamp>()?.as_millisecond(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Search {
    client: Client,
}

impl Search {
    pub async fn new() -> Result<Self> {
        let client = Client::new(&Settings::get().meilisearch_url, Some("masterkey"))?;

        let posts_index = client.index("posts");
        posts_index
            .set_settings(
                &MeilisearchSettings::new()
                    .with_searchable_attributes(["text"])
                    .with_filterable_attributes(["user_id"])
                    .with_sortable_attributes(["created_at"]),
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

    /// Search for posts with filtering and pagination
    ///
    /// * `search_params` - Search parameters including query text, user filter, and pagination options
    pub async fn post_search(&self, search_params: &PostSearch) -> Result<Vec<Post>> {
        let posts_index = self.client.index("posts");
        let mut search_query = posts_index.search();

        search_query.with_query(&search_params.query);

        let filter = search_params
            .user_id
            .as_ref()
            .map(|uid| format!("user_id = \"{}\"", uid));
        if let Some(filter) = filter.as_ref() {
            search_query.with_filter(filter);
        }

        if let Some(offset) = search_params.offset {
            search_query.with_offset(offset);
        }
        if let Some(limit) = search_params.limit {
            search_query.with_limit(limit);
        }

        let search_results = search_query.execute().await?;

        Ok(search_results
            .hits
            .into_iter()
            .map(|hit| hit.result)
            .collect())
    }
}

#[derive(Debug)]
pub struct PostSearch {
    pub query: String,
    pub user_id: Option<String>,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}
