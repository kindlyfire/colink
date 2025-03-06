use anyhow::Result;
use axum::{Json, Router, routing::get};
use ormlite::{Connection, sqlite::SqliteConnection};
use serde_json::{Value, json};
use sqlx::Row;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let app = Router::new().route("/", get(json));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind address");
    axum::serve(listener, app)
        .await
        .expect("server failed to start");

    Ok(())
}

async fn json() -> Json<Value> {
    let mut conn = SqliteConnection::connect(":memory:").await.unwrap();

    let res = sqlx::query("SELECT 1 + 1 + 4")
        .fetch_one(&mut conn)
        .await
        .unwrap();

    Json(json!({ "data": res.get::<i64, _>(0) }))
}
