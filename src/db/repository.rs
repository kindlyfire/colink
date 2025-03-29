use anyhow::Result;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

use crate::settings::Settings;

use super::{
    migrate::migrate_database,
    models::{links, post_links, posts, sessions, users},
};

#[derive(Clone, Debug)]
pub struct Repository {
    pub conn: sea_orm::DatabaseConnection,
}

impl Repository {
    pub async fn new() -> Result<Self> {
        let conn = create_database().await?;
        Ok(Self { conn })
    }

    pub async fn user_by_username(&self, username: &str) -> Result<Option<users::Model>> {
        let user = users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .one(&self.conn)
            .await?;
        Ok(user)
    }

    pub async fn user_by_id(&self, id: &str) -> Result<Option<users::Model>> {
        let user = users::Entity::find_by_id(id).one(&self.conn).await?;
        Ok(user)
    }

    pub async fn session_by_token(&self, token: &str) -> Result<Option<sessions::Model>> {
        let session = sessions::Entity::find()
            .filter(sessions::Column::Token.eq(token))
            .one(&self.conn)
            .await?;
        Ok(session)
    }

    pub async fn user_post_by_id(
        &self,
        post_id: &str,
        user_id: &str,
    ) -> Result<Option<posts::Model>> {
        let post = posts::Entity::find()
            .filter(posts::Column::Id.eq(post_id))
            .filter(posts::Column::UserId.eq(user_id))
            .one(&self.conn)
            .await?;
        Ok(post)
    }

    #[allow(unused)]
    pub async fn links_by_post_id(&self, post_id: &str) -> Result<Vec<links::Model>> {
        let links = post_links::Entity::find()
            .filter(post_links::Column::PostId.eq(post_id))
            .find_with_related(links::Entity)
            .all(&self.conn)
            .await?
            .into_iter()
            .flat_map(|(_, links)| links)
            .collect();
        Ok(links)
    }
}

async fn create_database() -> Result<sea_orm::DatabaseConnection> {
    let db_url = format!("sqlite:{}?mode=rwc", Settings::instance().db_url);
    let db = sea_orm::Database::connect(db_url).await?;

    db.execute(sea_orm::Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        "PRAGMA journal_mode = WAL;".to_string(),
    ))
    .await?;

    migrate_database(&db).await?;

    Ok(db)
}
