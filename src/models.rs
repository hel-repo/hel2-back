use chrono::{DateTime, Utc};

use db::models::types::{Language, UserGroup};

#[derive(Serialize, Deserialize)]
pub struct Localized {
    language: Language,
    text: String,
}

#[derive(Serialize, Deserialize)]
pub struct ContentNode {
    #[serde(rename = "type")]
    pub node_type: NodeType,
    pub path: String,
}

pub mod user {
    #[derive(Serialize, Deserialize)]
    pub struct Full {
        pub username: String,
        pub group: UserGroup,
        pub registered: DateTime<Utc>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Short {
        pub username: String,
    }

    pub enum User {
        Full(Full),
        Short(Short),
    }
}

pub mod package {
    #[derive(Serialize, Deserialize)]
    pub struct Full {
        pub name: String,
        pub description: Vec<Localized>,
        pub website: String,
        pub license: String,
        pub authors: Vec<String>,
        pub maintainers: Vec<user::Short>,
        pub versions: Vec<version::Full>,
        pub downloads: i32,
        pub likes: i32,
        pub created: DateTime<Utc>,
        pub updated: DateTime<Utc>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Short {
        pub name: String,
        pub description: Vec<Localized>,
        pub maintainers: Vec<user::Short>,
        pub versions: Vec<version::Short>,
        pub downloads: i32,
        pub likes: i32,
    }

    pub enum Package {
        Full(Full),
        Short(Short),
    }
}

pub mod version {
    pub struct Full {
        pub version: String,
        pub changes: Vec<Localized>,
        pub readme: Vec<Localized>,
        pub url: String,
        pub dependencies: Vec<dependency::Full>,
        pub contents: Vec<ContentNode>,
        pub created: DateTime<Utc>,
    }

    pub struct Short {
        pub version: String,
        pub url: String,
        pub dependencies: Vec<dependency::Short>,
    }

    pub enum Version {
        Full(Full),
        Short(Short),
    }
}

pub mod dependency {
    pub struct Full {
        pub package: package::Short,
        pub spec: String,
        #[serde(rename = "type")]
        pub dep_type: DependencyType,
        pub description: Option<Vec<Localized>>,
    }

    pub struct Short {
        pub package: String,
        pub spec: String,
    }

    pub enum Dependency {
        Full(Full),
        Short(Short),
    }
}