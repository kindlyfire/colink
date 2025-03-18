use std::collections::HashSet;

use anyhow::Result;
use rust_embed::Embed;
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement, TransactionTrait};
use tracing::debug;

use crate::db::now_utc;

pub async fn migrate_database(conn: &DatabaseConnection) -> Result<()> {
    let tx = conn.begin().await?;

    // Create migrations table if not exists
    tx.execute_unprepared(
        "CREATE TABLE IF NOT EXISTS migrations (
            name TEXT PRIMARY KEY NOT NULL,
            applied_at TEXT
        )",
    )
    .await?;

    // Get already applied migrations
    let applied_migrations: HashSet<String> = {
        tx.query_all(Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            "SELECT name FROM migrations",
        ))
        .await?
        .iter()
        .map(|res| res.try_get_by_index::<String>(0).unwrap())
        .collect()
    };
    debug!("Already applied migrations: {:?}", applied_migrations);

    // Run pending migrations
    let migrations = get_migrations();
    let pending_migrations = migrations
        .iter()
        .filter(|m| !applied_migrations.contains(&m.name))
        .collect::<Vec<&Migration>>();

    for migration in pending_migrations {
        // Apply migration
        tx.execute(Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            &migration.sql,
        ))
        .await?;

        // Record migration
        tx.execute(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Sqlite,
            "INSERT INTO migrations (name, applied_at) VALUES (?1, ?2)",
            [migration.name.to_string().into(), now_utc().into()],
        ))
        .await?;

        println!("Applied migration {}", migration.name);
    }

    tx.commit().await?;
    Ok(())
}

#[derive(Embed)]
#[folder = "src/db/migrations/"]
struct MigrationFile;

struct Migration {
    name: String,
    sql: String,
}

fn get_migrations() -> Vec<Migration> {
    let mut migrations = Vec::new();

    for file in MigrationFile::iter() {
        let migration = Migration {
            name: file.trim_end_matches(".sql").to_string(),
            sql: std::str::from_utf8(&MigrationFile::get(&file).unwrap().data)
                .unwrap_or_else(|_| panic!("migration file {file} should be valid utf8"))
                .to_owned(),
        };
        migrations.push(migration);
    }

    migrations
}
