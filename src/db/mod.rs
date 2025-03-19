mod migrate;
pub mod models;
pub mod repository;

use jiff::Timestamp;

/// IDs are prefixed with a type identifier to make them easy to distinguish.
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

    pub fn create(&self) -> String {
        format!("{}_{}", self.prefix(), cuid2::create_id())
    }
}

pub fn now_utc() -> String {
    Timestamp::now().to_string()
}
