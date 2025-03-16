use anyhow::Result;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod db;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind address");
    axum::serve(listener, routes::get_router())
        .await
        .expect("server failed to start");

    Ok(())
}
