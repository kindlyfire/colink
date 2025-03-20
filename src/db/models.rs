use sea_orm::entity::prelude::*;
use serde::Serialize;

pub mod users {

    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub created_at: String,
        pub updated_at: String,
        #[sea_orm(unique)]
        pub username: String,
        #[serde(skip_serializing)]
        pub password: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "sessions::Entity")]
        Sessions,
        #[sea_orm(has_many = "posts::Entity")]
        Posts,
        #[sea_orm(has_many = "links::Entity")]
        Links,
    }

    impl Related<sessions::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Sessions.def()
        }
    }

    impl Related<posts::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Posts.def()
        }
    }

    impl Related<links::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Links.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod sessions {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
    #[sea_orm(table_name = "sessions")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub created_at: String,
        pub updated_at: String,
        pub user_id: String,
        #[sea_orm(unique)]
        pub token: String,
        pub last_seen: String,
        pub label: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "users::Entity",
            from = "Column::UserId",
            to = "users::Column::Id"
        )]
        User,
    }

    impl Related<users::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::User.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod posts {
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
}

pub mod links {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
    #[sea_orm(table_name = "links")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub created_at: String,
        pub updated_at: String,
        pub user_id: String,
        pub url: String,
        pub title: Option<String>,
        pub html: Option<String>,
        pub text: Option<String>,
        pub scraped_at: Option<String>,
        pub scrape_pending: i32,
        pub scrape_error: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "post_links::Entity")]
        PostLinks,
        #[sea_orm(
            belongs_to = "users::Entity",
            from = "Column::UserId",
            to = "users::Column::Id"
        )]
        User,
    }

    impl Related<post_links::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::PostLinks.def()
        }
    }

    impl Related<users::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::User.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod post_links {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
    #[sea_orm(table_name = "post_links")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub created_at: String,
        pub updated_at: String,
        pub post_id: String,
        pub link_id: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "posts::Entity",
            from = "Column::PostId",
            to = "posts::Column::Id"
        )]
        Post,
        #[sea_orm(
            belongs_to = "links::Entity",
            from = "Column::LinkId",
            to = "links::Column::Id"
        )]
        Link,
    }

    impl Related<posts::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Post.def()
        }
    }

    impl Related<links::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Link.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}
