mod migrate;
pub mod models;

use anyhow::Result;
use jiff::Timestamp;
pub use migrate::migrate_database;
use sea_orm::ConnectionTrait;

pub async fn create_database() -> Result<sea_orm::DatabaseConnection> {
    let db_url = format!(
        "sqlite:{}?mode=rwc",
        std::env::var("DATABASE_URL").unwrap_or_else(|_| ":memory:".to_string())
    );
    let db = sea_orm::Database::connect(db_url).await?;

    db.execute(sea_orm::Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        "PRAGMA journal_mode = WAL;".to_string(),
    ))
    .await?;
    migrate_database(&db).await?;

    Ok(db)
}

/// Used to specify the type of ID to create, as IDs are prefixed with a letter.
#[allow(unused)]
pub enum IdType {
    User,
    Session,
    Post,
    Link,
    PostLink,
}

impl IdType {
    pub fn prefix(&self) -> &'static str {
        match self {
            IdType::User => "u",
            IdType::Session => "s",
            IdType::Post => "p",
            IdType::Link => "l",
            IdType::PostLink => "pl",
        }
    }
}

/// Creates a new ID with the specified type encoded as a prefix.
pub fn create_id(typ: IdType) -> String {
    format!("{}_{}", typ.prefix(), cuid2::create_id())
}

pub fn now_utc() -> String {
    Timestamp::now().to_string()
}
