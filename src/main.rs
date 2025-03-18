use anyhow::{Result, anyhow};
use bcrypt::{DEFAULT_COST, hash};
use clap::{Parser, Subcommand};
use db::{IdType, create_database, create_id, models::users, now_utc};
use routes::AppState;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use std::env;
use tokio::net::TcpListener;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

mod db;
mod routes;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Set log level to debug
    #[arg(short, long)]
    v: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Start the server
    Serve {
        /// Host to bind the server to (overrides HOST env var)
        #[arg(long, value_name = "HOST")]
        host: Option<String>,

        /// Port to bind the server to (overrides PORT env var)
        #[arg(long, value_name = "PORT")]
        port: Option<u16>,
    },

    /// User management commands
    Users {
        #[command(subcommand)]
        command: UserCommand,
    },
}

#[derive(Subcommand)]
enum UserCommand {
    /// Create a new user
    Create {
        /// Username for the new user
        username: String,

        /// Password for the new user
        password: String,
    },

    /// Reset a user's password
    SetPassword {
        /// Username of the user
        username: String,

        /// New password for the user
        password: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.v { Level::DEBUG } else { Level::INFO };
    let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    match cli.command {
        Command::Serve { host, port } => cmd_serve(host, port).await?,
        Command::Users { command } => cmd_users(command).await?,
    }

    Ok(())
}

async fn cmd_serve(host: Option<String>, port: Option<u16>) -> Result<()> {
    let db = create_database().await?;

    // Get host from flag, environment variable, or default
    let host = host
        .or_else(|| env::var("HOST").ok())
        .unwrap_or_else(|| "0.0.0.0".to_string());

    // Get port from flag, environment variable, or default
    let port = port
        .or_else(|| env::var("PORT").ok().and_then(|p| p.parse().ok()))
        .unwrap_or(3000);

    let listener = TcpListener::bind(&format!("{}:{}", host, port))
        .await
        .expect("failed to bind address");
    let router = routes::get_router(AppState { db });

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router)
        .await
        .expect("server failed to start");

    Ok(())
}

async fn cmd_users(command: UserCommand) -> Result<()> {
    let db = create_database().await?;

    match command {
        UserCommand::Create { username, password } => {
            // Check if user already exists
            let existing_user = users::Entity::find()
                .filter(users::Column::Username.eq(&username))
                .one(&db)
                .await?;
            if existing_user.is_some() {
                return Err(anyhow!("User already exists"));
            }

            // Create new user
            let now = now_utc();
            let user = users::ActiveModel {
                id: Set(create_id(IdType::User)),
                username: Set(username),
                password: Set(hash(password, DEFAULT_COST)?),
                created_at: Set(now.clone()),
                updated_at: Set(now),
            };

            let user = user.insert(&db).await?;
            info!("Created user: {:?}", user);
        }
        UserCommand::SetPassword { username, password } => {
            // Find the user
            let user = users::Entity::find()
                .filter(users::Column::Username.eq(&username))
                .one(&db)
                .await?
                .ok_or_else(|| anyhow!("User not found"))?;

            // Update the password
            let mut user_model: users::ActiveModel = user.into();
            user_model.password = Set(hash(password, DEFAULT_COST)?);
            user_model.updated_at = Set(now_utc());
            user_model.update(&db).await?;

            info!("Updated password for user: {}", username);
        }
    };

    Ok(())
}
