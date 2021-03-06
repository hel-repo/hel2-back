use chrono::NaiveDateTime;
use diesel::sql_types::{Text, BigInt};

use super::schema::*;

pub mod types {
    use std::{io, fmt};
    use std::str::FromStr;

    use diesel::backend::Backend;
    use diesel::deserialize::{self, FromSql};
    use diesel::serialize::{self, Output, ToSql};
    use diesel::sql_types::Varchar;

    use error::ParseError;

    #[derive(AsExpression, FromSqlRow, Serialize, Deserialize,
             Debug, Copy, Clone, Eq, PartialEq, Hash)]
    #[sql_type = "Varchar"]
    pub enum UserGroup {
        User,
        Admin,
        Banned,
    }

    impl FromStr for UserGroup {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "user" => Ok(UserGroup::User),
                "admin" => Ok(UserGroup::Admin),
                "banned" => Ok(UserGroup::Banned),
                _ => Err(ParseError::UnknownGroup { group: s.to_string() } ),
            }
        }
    }

    impl fmt::Display for UserGroup {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", match *self {
                UserGroup::User => "user",
                UserGroup::Admin => "admin",
                UserGroup::Banned => "banned",
            })
        }
    }

    impl<DB: Backend> ToSql<Varchar, DB> for UserGroup
    where
        String: ToSql<Varchar, DB>,
    {
        fn to_sql<W: io::Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
            self.to_string().to_sql(out)
        }
    }

    impl<DB: Backend> FromSql<Varchar, DB> for UserGroup
    where
        String: FromSql<Varchar, DB>,
    {
        fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
            String::from_sql(bytes)?.parse::<UserGroup>()
                .map_err(|e| e.to_string().into())
        }
    }

    #[derive(AsExpression, FromSqlRow, Serialize, Deserialize,
             Debug, Copy, Clone, Eq, PartialEq, Hash)]
    #[sql_type = "Varchar"]
    pub enum DependencyType {
        BuildRequire,
        RuntimeRequire,
        Optional,
    }

    impl FromStr for DependencyType {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "build-require" => Ok(DependencyType::BuildRequire),
                "runtime-require" => Ok(DependencyType::RuntimeRequire),
                "optional" => Ok(DependencyType::Optional),
                _ => Err(ParseError::UnknownDependencyType { name: s.to_string()}),
            }
        }
    }

    impl fmt::Display for DependencyType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", match *self {
                DependencyType::BuildRequire => "build-require",
                DependencyType::RuntimeRequire => "runtime-require",
                DependencyType::Optional => "optional",
            })
        }
    }

    impl<DB: Backend> ToSql<Varchar, DB> for DependencyType
    where
        String: ToSql<Varchar, DB>,
    {
        fn to_sql<W: io::Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
            self.to_string().to_sql(out)
        }
    }

    impl<DB: Backend> FromSql<Varchar, DB> for DependencyType
    where
        String: FromSql<Varchar, DB>,
    {
        fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
            String::from_sql(bytes)?.parse::<DependencyType>()
                .map_err(|e| e.to_string().into())
        }
    }

    #[derive(AsExpression, FromSqlRow, Serialize, Deserialize,
             Debug, Copy, Clone, Eq, PartialEq, Hash)]
    #[sql_type = "Varchar"]
    pub enum NodeType {
        File,
        Directory,
    }

    impl FromStr for NodeType {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "f" | "file" => Ok(NodeType::File),
                "d" | "dir" | "directory" => Ok(NodeType::Directory),
                _ => Err(ParseError::UnknownNodeType { name: s.to_string() }),
            }
        }
    }

    impl fmt::Display for NodeType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", match *self {
                NodeType::File => "file",
                NodeType::Directory => "dir",
            })
        }
    }

    impl<DB: Backend> ToSql<Varchar, DB> for NodeType
    where
        String: ToSql<Varchar, DB>,
    {
        fn to_sql<W: io::Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
            self.to_string().to_sql(out)
        }
    }

    impl<DB: Backend> FromSql<Varchar, DB> for NodeType
    where
        String: FromSql<Varchar, DB>,
    {
        fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
            String::from_sql(bytes)?.parse::<NodeType>()
                .map_err(|e| e.to_string().into())
        }
    }

    #[derive(AsExpression, FromSqlRow, Serialize, Deserialize,
             Debug, Copy, Clone, Eq, PartialEq, Hash)]
    #[sql_type = "Varchar"]
    pub enum Language {
        Russian,
        English,
    }

    impl FromStr for Language {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "ru" | "rus" | "russian" => Ok(Language::Russian),
                "en" | "eng" | "english" => Ok(Language::English),
                _ => Err(ParseError::UnknownLanguage { language: s.to_string() }),
            }
        }
    }

    impl fmt::Display for Language {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", match *self {
                Language::Russian => "ru",
                Language::English => "en",
            })
        }
    }

    impl<DB: Backend> ToSql<Varchar, DB> for Language
    where
        String: ToSql<Varchar, DB>,
    {
        fn to_sql<W: io::Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
            self.to_string().to_sql(out)
        }
    }

    impl<DB: Backend> FromSql<Varchar, DB> for Language
    where
        String: FromSql<Varchar, DB>,
    {
        fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
            String::from_sql(bytes)?.parse::<Language>()
                .map_err(|e| e.to_string().into())
        }
    }
}

#[derive(Queryable, Identifiable, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: Vec<u8>,
    pub salt: Vec<u8>,
    pub group: types::UserGroup,
    pub registered: NaiveDateTime,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a [u8],
    pub salt: &'a [u8],
    pub group: types::UserGroup,
}

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[primary_key(name)]
pub struct Package {
    pub name: String,
    pub website: String,
    pub license: String,
    pub authors: Vec<String>,
    pub downloads: i32,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "packages"]
pub struct NewPackage<'a> {
    pub name: &'a str,
    pub website: &'a str,
    pub license: &'a str,
    pub authors: &'a Vec<String>,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[primary_key(user, package)]
#[belongs_to(User, foreign_key = "user")]
#[belongs_to(Package, foreign_key = "package")]
pub struct Like {
    pub user: i32,
    pub package: String,
    pub time: NaiveDateTime,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "likes"]
pub struct NewLike<'a> {
    pub user: i32,
    pub package: &'a str,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[belongs_to(Package, foreign_key = "package")]
pub struct Version {
    pub id: i32,
    pub package: String,
    pub version: String,
    pub created: NaiveDateTime,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "versions"]
pub struct NewVersion<'a> {
    pub package: &'a str,
    pub version: &'a str,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[table_name = "dependencies"]
#[belongs_to(Package, foreign_key = "package")]
#[belongs_to(Version, foreign_key = "version")]
pub struct Dependency {
    pub id: i32,
    pub package: String,
    pub version: i32,
    pub spec: String,
    #[column_name = "type_"]
    pub dep_type: types::DependencyType,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "dependencies"]
pub struct NewDependency<'a> {
    pub package: &'a str,
    pub version: i32,
    pub spec: &'a str,
    #[column_name = "type_"]
    pub dep_type: types::DependencyType,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[belongs_to(Version, foreign_key = "version")]
#[table_name = "contents"]
pub struct ContentNode {
    pub id: i32,
    pub version: i32,
    pub path: String,
    #[column_name = "type_"]
    pub node_type: types::NodeType,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "contents"]
pub struct NewContentNode<'a> {
    pub version: i32,
    pub path: &'a str,
    #[column_name = "type_"]
    pub node_type: types::NodeType,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[primary_key(user, package)]
#[belongs_to(User, foreign_key = "user")]
#[belongs_to(Package, foreign_key = "package")]
pub struct Maintainer {
    pub user: i32,
    pub package: String,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "maintainers"]
pub struct NewMaintainer<'a> {
    pub user: i32,
    pub package: &'a str,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[primary_key(package, language)]
#[belongs_to(Package, foreign_key = "package")]
pub struct Description {
    pub package: String,
    pub language: types::Language,
    pub description: String,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "descriptions"]
pub struct NewDescription<'a> {
    pub package: &'a str,
    pub language: types::Language,
    pub description: &'a str,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[primary_key(version, language)]
#[belongs_to(Version, foreign_key = "version")]
pub struct VersionText {
    pub version: i32,
    pub language: types::Language,
    pub changes: String,
    pub readme: String,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "version_texts"]
pub struct NewVersionText<'a> {
    pub version: i32,
    pub language: types::Language,
    pub changes: &'a str,
    pub readme: &'a str,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[primary_key(dependency, language)]
#[belongs_to(Dependency, foreign_key = "dependency")]
pub struct DependencyDescription {
    pub dependency: i32,
    pub language: types::Language,
    pub description: String,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "dependency_descriptions"]
pub struct NewDependencyDescription<'a> {
    pub dependency: i32,
    pub language: types::Language,
    pub description: &'a str,
}

#[derive(Queryable, QueryableByName, PartialEq, Debug)]
pub struct GroupedLike {
    #[sql_type = "Text"]
    pub package: String,
    #[sql_type = "BigInt"]
    pub likes: i64,
}