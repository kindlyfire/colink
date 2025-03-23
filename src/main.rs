use anyhow::{Result, anyhow};
use bcrypt::{DEFAULT_COST, hash};
use clap::{Parser, Subcommand};
use db::{IdType, models::users, now_utc, repository::Repository};
use routes::AppState;
use sea_orm::{ActiveModelTrait, Set};
use search::Search;
use std::env;
use tokio::net::TcpListener;
use tracing::{Level, info};
use tracing_subscriber::{EnvFilter, prelude::*};

mod db;
mod routes;
mod search;

#[cfg(test)]
mod tests;

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
    let filter = if cli.v {
        EnvFilter::new(format!("{}", log_level))
    } else {
        EnvFilter::new("info")
            .add_directive("colink=info".parse().unwrap())
            .add_directive("warn".parse().unwrap())
    };

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    match cli.command {
        Command::Serve { host, port } => cmd_serve(host, port).await?,
        Command::Users { command } => cmd_users(command).await?,
    }

    Ok(())
}

async fn cmd_serve(host: Option<String>, port: Option<u16>) -> Result<()> {
    // Get host from flag, environment variable, or default
    let host = host
        .or_else(|| env::var("HOST").ok())
        .unwrap_or_else(|| "0.0.0.0".to_string());

    // Get port from flag, environment variable, or default
    let port = port
        .or_else(|| env::var("PORT").ok().and_then(|p| p.parse().ok()))
        .unwrap_or(3000);

    let repo = Repository::new().await?;
    let search = Search::new().await?;

    let listener = TcpListener::bind(&format!("{}:{}", host, port))
        .await
        .expect("failed to bind address");
    let router = routes::get_router(AppState {
        repo,
        search: Some(search),
    });

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router)
        .await
        .expect("server failed to start");

    Ok(())
}

async fn cmd_users(command: UserCommand) -> Result<()> {
    let repo = Repository::new().await?;

    match command {
        UserCommand::Create { username, password } => {
            // Check if user already exists
            let existing_user = repo.user_by_username(&username).await?;
            if existing_user.is_some() {
                return Err(anyhow!("User already exists"));
            }

            // Create new user
            let now = now_utc();
            let user = users::ActiveModel {
                id: Set(IdType::User.create()),
                username: Set(username),
                password: Set(hash(password, DEFAULT_COST)?),
                created_at: Set(now.clone()),
                updated_at: Set(now),
            };

            let user = user.insert(&repo.conn).await?;
            info!("Created user: {:?}", user);
        }
        UserCommand::SetPassword { username, password } => {
            // Find the user
            let user = repo
                .user_by_username(&username)
                .await?
                .ok_or_else(|| anyhow!("User not found"))?;

            // Update the password
            let mut user_model: users::ActiveModel = user.into();
            user_model.password = Set(hash(password, DEFAULT_COST)?);
            user_model.updated_at = Set(now_utc());
            user_model.update(&repo.conn).await?;

            info!("Updated password for user: {}", username);
        }
    };

    Ok(())
}
