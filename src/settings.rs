use std::env;
use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct Settings {
    pub db_url: String,
    pub meilisearch_url: String,
    pub meilisearch_key: Option<String>,
    pub host: String,
    pub port: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            db_url: String::from("db.sqlite3"),
            meilisearch_url: String::from("http://127.0.0.1:7700"),
            meilisearch_key: None,
            host: String::from("localhost"),
            port: 3000,
        }
    }
}

impl Settings {
    fn new() -> Self {
        let mut settings = Settings::default();
        if let Ok(db_url) = env::var("DB_URL") {
            settings.db_url = db_url.replace("sqlite://", "").replace("sqlite:", "");
        }
        if let Ok(meilisearch_url) = env::var("MEILISEARCH_URL") {
            settings.meilisearch_url = meilisearch_url;
        }
        if let Ok(meilisearch_key) = env::var("MEILISEARCH_KEY") {
            settings.meilisearch_key = (!meilisearch_key.is_empty()).then_some(meilisearch_key)
        }
        if let Ok(host) = env::var("HOST") {
            settings.host = host;
        }
        if let Ok(port) = env::var("PORT") {
            settings.port = port
                .parse()
                .expect("Invalid port number in env variable PORT");
        }
        settings
    }

    pub fn instance() -> &'static Settings {
        static INSTANCE: LazyLock<Settings> = LazyLock::new(Settings::new);
        &INSTANCE
    }
}
