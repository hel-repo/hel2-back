use schema;

pub enum DependencyType {
    BuildRequire,
    RuntimeRequire,
    Optional
}

impl<'a> FromStr for DependencyType {
    type Err = &'a str;

    fn from_str(s: &'a str) -> Result<Self, Self::Err> {
        match s {
            "build-require" => Ok(DependencyType::BuildRequire),
            "runtime-require" => Ok(DependencyType::RuntimeRequire),
            "optional" => Ok(DependencyType::Optional),
            _ => Err(format!("Unknown dependency type: {}", s)),
        }
    }
}

pub enum NodeType {
    File,
    Directory
}

impl<'a> FromStr for NodeType {
    type Err = &'a str;

    fn from_str(s: &'a str) -> Result(Self, Self::Err) {
        match s {
            "f" | "file" => Ok(NodeType::File),
            "d" | "dir" | "directory" => Ok(NodeType::Directory),
            _ => Err(format!("Unknown node type: {}", s)),
        }
    }
}

pub enum Language {
    Russian,
    English,
}

impl<'a> FromStr for Language {
    type Err = &'a str;

    fn from_str(s: &'a str) -> Result(Self, Self::Err) {
        match s {
            "ru" | "rus" | "russian" => Ok(Language::Russian),
            "en" | "eng" | "english" => Ok(Language::English),
            _ => Err(format!("Unknown language: {}", s)),
        }
    }
}

#[derive(Queryable, Identifiable, PartialEq, Debug)]
pub struct User {
    pub id: i32,
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
}

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[belongs_to(Package, foreign_key = "package")]
pub struct Version {
    pub id: i32,
    pub package: String,
    pub version: Text,
}

#[derive(Identifiable, PartialEq, Debug)]
#[table_name = "dependencies"]
#[belongs_to(Package, foreign_key = "package")]
#[belongs_to(Version, foreign_key = "version")]
pub struct Dependency {
    pub id: i32,
    pub package: String,
    pub version: i32,
    pub spec: String,
    pub dep_type: DependencyType,
}

impl Queryable<schema::dependencies::SqlType, DB> for Dependency {
    type Row = (i32, String, i32, String, String);

    fn build(row: Self::Row) -> Self {
        Dependency {
            id: row.0,
            package: row.1,
            version: row.2,
            spec: row.3,
            dep_type: row.4.parse::<DependencyType>().unwrap(),
        }
    }
}

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[belongs_to(Version, foreign_key = "version")]
#[table_name = "contents"]
pub struct ContentNode {
    pub id: i32,
    pub version: i32,
    pub path: String,
    pub node_type: NodeType,
}

impl Queryable<schema::contents::SqlType, DB> for ContentNode {
    type Row = (i32, i32, String, String);

    fn build(row: self::Row) -> Self {
        ContentNode {
            id: row.0,
            version: row.1,
            path: row.2,
            node_type: row.3.parse::<NodeType>().unwrap(),
        }
    }
}

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[primary_key(user, package)]
#[belongs_to(User, foreign_key = "user")]
#[belongs_to(Package, foreign_key = "package")]
pub struct Maintainer {
    pub user: i32,
    pub package: String,
}

#[derive(Identifiable, PartialEq, Debug)]
#[primary_key(package, language)]
#[belongs_to(Package, foreign_key = "package")]
pub struct Description {
    pub package: String,
    pub language: Language,
    pub description: String,
}

impl Queryable<schema::descriptions::SqlType, DB> for Description {
    type Row = (String, String, String);

    fn build(row: Self::Row) -> Self {
        Description {
            package: row.0,
            language: row.1.parse::<Language>().unwrap(),
            description: row.2,
        }
    }
}

#[derive(Identifiable, PartialEq, Debug)]
#[primary_key(version, language)]
#[belongs_to(Version, foreign_key = "version")]
pub struct VersionText {
    pub version: i32,
    pub language: String,
    pub changes: String,
    pub readme: String,
}

impl Queryable<schema::version_texts::SqlType, DB> for VersionText {
    type Row = (i32, String, String, String);

    fn build(row: Self::Row) -> Self {
        VersionText {
            version: row.0,
            language: row.1.parse::<Language>().unwrap(),
            changes: row.2,
            readme: row.3,
        }
    }
}

#[derive(Identifiable, PartialEq, Debug)]
#[primary_key(dependency, language)]
#[belongs_to(Dependency, foreign_key = "dependency")]
pub struct DependencyDescription {
    pub dependency: i32,
    pub language: String,
    pub description: String,
}

impl Queryable<schema::dependency_descriptions::SqlType, DB> for DependencyDescription {
    type Row = (i32, String, String);

    fn build(row: Self::Row) -> Self {
        DependencyDescription {
            dependency: row.0,
            language: row.1.parse::<Language>().unwrap(),
            description: row.2,
        }
    }
}