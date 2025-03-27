use crate::db::{IdType, now_utc, repository::Repository};
use anyhow::Result;
use regex::Regex;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, Set, TransactionTrait,
};
use std::collections::HashSet;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub user_id: String,
    pub text: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "users::Entity",
        from = "Column::UserId",
        to = "users::Column::Id"
    )]
    User,
    #[sea_orm(has_many = "post_links::Entity")]
    PostLinks,
}

impl Related<users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<post_links::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostLinks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Syncs the links present in the post to the database by
    /// creating/updating/deleting post_links and links.
    pub async fn update_links(&self, repo: &Repository) -> Result<()> {
        let found_urls = find_urls(&self.text);

        let txn = repo.conn.begin().await?;

        // Get existing post links along with their link data in a single query
        let existing_links_data = post_links::Entity::find()
            .filter(post_links::Column::PostId.eq(&self.id))
            .find_with_related(links::Entity)
            .all(&txn)
            .await?;

        let existing_link_urls: HashSet<String> = existing_links_data
            .iter()
            .filter_map(|(_, links)| links.first().map(|link| link.url.clone()))
            .collect();

        // Find URLs to add (in found_urls but not in existing_link_urls)
        let urls_to_add: Vec<String> = found_urls
            .difference(&existing_link_urls)
            .cloned()
            .collect();

        // Add new links and create post-link associations
        for url in urls_to_add {
            let new_link = links::ActiveModel {
                id: Set(IdType::Link.create()),
                created_at: Set(now_utc()),
                updated_at: Set(now_utc()),
                user_id: Set(self.user_id.clone()),
                url: Set(url),
                title: Set(None),
                html: Set(None),
                text: Set(None),
                scraped_at: Set(None),
                scrape_pending: Set(1),
                scrape_error: Set(None),
            };
            let link_result = new_link.insert(&txn).await?;

            let new_post_link = post_links::ActiveModel {
                id: Set(IdType::PostLink.create()),
                created_at: Set(now_utc()),
                updated_at: Set(now_utc()),
                post_id: Set(self.id.clone()),
                link_id: Set(link_result.id.clone()),
            };
            new_post_link.insert(&txn).await?;
        }

        let urls_to_remove: Vec<String> = existing_link_urls
            .difference(&found_urls)
            .cloned()
            .collect();
        for url_to_remove in urls_to_remove {
            let post_link = existing_links_data
                .iter()
                .find(|(_, links)| {
                    if let Some(link) = links.first() {
                        link.url == url_to_remove
                    } else {
                        false
                    }
                })
                .ok_or_else(|| anyhow::anyhow!("Post link not found"))?;
            post_link.0.clone().delete(&txn).await?;
        }

        txn.commit().await?;

        Ok(())
    }
}

/// Extracts URLs from text, supporting both plain URLs and Markdown link
/// format.
///
/// It works in two stages:
/// 1. Extract the Markdown links and remove them from the string.
/// 2. Extract the plain URLs from the remaining string.
///
/// The function removes markdown links from the text to avoid double-matching,
/// and it's quite lax in allowed URLs.
///
/// NOTE: This doesn't handle some edge-cases, like parentheses in URLs. If it
/// becomes a problem I'll hand-roll a parser at some point.
pub fn find_urls(text: &str) -> HashSet<String> {
    let mut urls = HashSet::new();
    let text = text.to_string();

    // First, extract Markdown links: [text](url)
    let markdown_regex = Regex::new(r"\[([^\]]*)\]\(([^)]+)\)").unwrap();
    for cap in markdown_regex.captures_iter(&text) {
        if let Some(url_match) = cap.get(2) {
            let url = url_match.as_str().to_string();
            urls.insert(url);
        }
    }

    // Remove markdown links from the text to avoid double matching
    let text = markdown_regex.replace_all(&text, "").to_string();

    // Now extract plain URLs
    let url_regex = Regex::new(r"(https?://[^\s]+)").unwrap();
    for cap in url_regex.captures_iter(&text) {
        if let Some(url_match) = cap.get(1) {
            let url = url_match.as_str().to_string();
            // Remove trailing punctuation that might have been captured
            let clean_url = url.trim_end_matches(|c| ",.:;!?'\"".contains(c));
            urls.insert(clean_url.to_string());
        }
    }

    urls
}

#[test]
pub fn test_find_urls() {
    let urls = vec![
        "https://example.com",
        "http://example.com",
        "http://example.com/apathandstuff",
        "http://example.c/?blah=true+test#hash",
    ];

    for url in urls {
        let res = find_urls(&format!(
            "Check this out: {} and also this: [link]({}a)",
            url, url
        ));
        println!("{:?}", res);
        assert!(res.contains(url));
        assert!(res.contains(&format!("{}a", url)));
    }
}
