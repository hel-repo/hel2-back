use ::db::models::types::{Language, NodeType};

pub mod date_serde {
    use chrono::{NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%+";

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Localized {
    pub language: Language,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct ContentNode {
    #[serde(rename = "type")]
    pub node_type: NodeType,
    pub path: String,
}

pub mod user {
    use chrono::NaiveDateTime;

    use ::db::models::types::UserGroup;

    #[derive(Serialize, Deserialize)]
    pub struct Full {
        pub username: String,
        pub group: UserGroup,
        #[serde(with = "super::date_serde")]
        pub registered: NaiveDateTime,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Short {
        pub username: String,
    }
}

pub mod package {
    use chrono::NaiveDateTime;

    use super::Localized;

    #[derive(Serialize, Deserialize)]
    pub struct Full {
        pub name: String,
        pub description: Vec<Localized>,
        pub website: String,
        pub license: String,
        pub authors: Vec<String>,
        pub maintainers: Vec<super::user::Short>,
        pub versions: Vec<super::version::Full>,
        pub downloads: i32,
        pub likes: i32,
        #[serde(with = "super::date_serde")]
        pub created: NaiveDateTime,
        #[serde(with = "super::date_serde")]
        pub updated: NaiveDateTime,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Short {
        pub name: String,
        pub description: Vec<Localized>,
        pub maintainers: Vec<super::user::Short>,
        pub versions: Vec<super::version::Short>,
        pub downloads: i32,
        pub likes: i32,
    }
}

pub mod version {
    use chrono::NaiveDateTime;

    use super::{ContentNode, Localized};

    #[derive(Serialize, Deserialize)]
    pub struct Full {
        pub version: String,
        pub changes: Vec<Localized>,
        pub readme: Vec<Localized>,
        pub url: String,
        pub dependencies: Vec<super::dependency::Full>,
        pub contents: Vec<ContentNode>,
        #[serde(with = "super::date_serde")]
        pub created: NaiveDateTime,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Short {
        pub version: String,
        pub url: String,
        pub dependencies: Vec<super::dependency::Short>,
    }
}

pub mod dependency {
    use ::db::models::types::DependencyType;

    use super::Localized;

    #[derive(Serialize, Deserialize)]
    pub struct Full {
        // package::Short would be nice here...
        pub package: String,
        pub spec: String,
        #[serde(rename = "type")]
        pub dep_type: DependencyType,
        pub description: Option<Vec<Localized>>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Short {
        pub package: String,
        pub spec: String,
    }
}

pub mod api {
    #[derive(Deserialize)]
    pub struct PaginationRq {
        pub page: u32,
        pub limit: u32,
    }

    impl PaginationRq {
        pub fn validate(self, page_limit: u32) -> PaginationRq {
            PaginationRq {
                page: self.page.max(1),
                limit: self.limit.min(page_limit).max(1),
            }
        }
    }

    #[derive(Deserialize)]
    pub struct Name {
        pub name: String,
    }
}