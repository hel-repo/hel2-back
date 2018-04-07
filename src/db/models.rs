use chrono::{DateTime, Utc};

use super::schema::*;

pub mod types {
    use std::{io, fmt};
    use std::str::FromStr;

    use diesel::backend::Backend;
    use diesel::deserialize::{self, FromSql};
    use diesel::serialize::{self, Output, ToSql};
    use diesel::sql_types::Varchar;

    use error::{Error, ErrorKind, Result};

    #[derive(AsExpression, FromSqlRow, Debug, Copy, Clone, Eq, PartialEq, Hash)]
    #[sql_type = "Varchar"]
    pub enum UserGroup {
        User,
        Admin,
        Banned,
    }

    impl FromStr for UserGroup {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self> {
            match s {
                "user" => Ok(UserGroup::User),
                "admin" => Ok(UserGroup::Admin),
                "banned" => Ok(UserGroup::Banned),
                _ => bail!(ErrorKind::UnknownGroup(s.to_string())),
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
            match String::from_sql(bytes)?.parse::<UserGroup>() {
                Ok(v) => Ok(v),
                Err(e) => Err(Box::new(e)),
            }
        }
    }

    #[derive(AsExpression, FromSqlRow, Debug, Copy, Clone, Eq, PartialEq, Hash)]
    #[sql_type = "Varchar"]
    pub enum DependencyType {
        BuildRequire,
        RuntimeRequire,
        Optional,
    }

    impl FromStr for DependencyType {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self> {
            match s {
                "build-require" => Ok(DependencyType::BuildRequire),
                "runtime-require" => Ok(DependencyType::RuntimeRequire),
                "optional" => Ok(DependencyType::Optional),
                _ => bail!(ErrorKind::UnknownDependencyType(s.to_string())),
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
            match String::from_sql(bytes)?.parse::<DependencyType>() {
                Ok(v) => Ok(v),
                Err(e) => Err(Box::new(e)),
            }
        }
    }

    #[derive(AsExpression, FromSqlRow, Debug, Copy, Clone, Eq, PartialEq, Hash)]
    #[sql_type = "Varchar"]
    pub enum NodeType {
        File,
        Directory,
    }

    impl FromStr for NodeType {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self> {
            match s {
                "f" | "file" => Ok(NodeType::File),
                "d" | "dir" | "directory" => Ok(NodeType::Directory),
                _ => bail!(ErrorKind::UnknownNodeType(s.to_string())),
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
            match String::from_sql(bytes)?.parse::<NodeType>() {
                Ok(v) => Ok(v),
                Err(e) => Err(Box::new(e)),
            }
        }
    }

    #[derive(AsExpression, FromSqlRow, Debug, Copy, Clone, Eq, PartialEq, Hash)]
    #[sql_type = "Varchar"]
    pub enum Language {
        Russian,
        English,
    }

    impl FromStr for Language {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self> {
            match s {
                "ru" | "rus" | "russian" => Ok(Language::Russian),
                "en" | "eng" | "english" => Ok(Language::English),
                _ => bail!(ErrorKind::UnknownLanguage(s.to_string())),
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
            match String::from_sql(bytes)?.parse::<Language>() {
                Ok(v) => Ok(v),
                Err(e) => Err(Box::new(e)),
            }
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
    pub registered: DateTime<Utc>,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: Vec<u8>,
    pub salt: Vec<u8>,
    pub group: String,
}

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[primary_key(name)]
pub struct Package {
    pub name: String,
    pub website: String,
    pub license: String,
    pub authors: Vec<String>,
    pub downloads: i32,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "packages"]
pub struct NewPackage {
    pub name: String,
    pub website: String,
    pub license: String,
    pub authors: Vec<String>,
    pub downloads: i32,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[primary_key(user, package)]
#[belongs_to(User, foreign_key = "user")]
#[belongs_to(Package, foreign_key = "package")]
pub struct Like {
    pub user: i32,
    pub package: String,
    pub time: DateTime<Utc>,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "likes"]
pub struct NewLike {
    pub user: i32,
    pub package: String,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[belongs_to(Package, foreign_key = "package")]
pub struct Version {
    pub id: i32,
    pub package: String,
    pub version: String,
    pub created: DateTime<Utc>,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "versions"]
pub struct NewVersion {
    pub package: String,
    pub version: String,
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
pub struct NewDependency {
    pub package: String,
    pub version: i32,
    pub spec: String,
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
pub struct NewContentNode {
    pub version: i32,
    pub path: String,
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
pub struct NewMaintainer {
    pub user: i32,
    pub package: String,
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
pub struct NewDescription {
    pub package: String,
    pub language: types::Language,
    pub description: String,
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
pub struct NewVersionText {
    pub version: i32,
    pub language: types::Language,
    pub changes: String,
    pub readme: String,
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
pub struct NewDependencyDescription {
    pub dependency: i32,
    pub language: types::Language,
    pub description: String,
}