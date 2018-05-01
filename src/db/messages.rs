use std::collections::HashMap;

use actix::{Message, Handler};
use diesel::{self, insert_into};
use diesel::sql_types::{BigInt, Text};
use diesel::prelude::*;
use failure::Error;

use ::models::*;
use super::DbExecutor;
use super::models;
use super::schema;

sql_function!(lower, lower_t, (s: Text) -> Text);

pub struct GetPackage(pub String);

impl Message for GetPackage {
    type Result = Result<package::Full, Error>;
}

impl Handler<GetPackage> for DbExecutor {
    type Result = Result<package::Full, Error>;

    fn handle(&mut self, msg: GetPackage, _: &mut Self::Context) -> Self::Result {
        let name = &msg.0;

        let package: models::Package = schema::packages::table.find(name)
            .get_result::<models::Package>(&self.conn)?;
        let likes: i64 = models::Like::belonging_to(&package)
            .count()
            .get_result(&self.conn)?;
        let likes: i32 = likes as i32;
        let versions: Vec<models::Version> = models::Version::belonging_to(&package)
            .load::<models::Version>(&self.conn)?;
        let dependencies: Vec<models::Dependency> = models::Dependency::belonging_to(&versions)
            .load::<models::Dependency>(&self.conn)?;
        let contents: Vec<models::ContentNode> = models::ContentNode::belonging_to(&versions)
            .load::<models::ContentNode>(&self.conn)?;
        let maintainers: Vec<String> = schema::users::table
            .inner_join(schema::maintainers::table.on(
                schema::users::id.eq(schema::maintainers::user).and(
                    schema::maintainers::package.eq(name)
                )
            ))
            .select(schema::users::username)
            .load::<String>(&self.conn)?;
        let descriptions: Vec<models::Description> = models::Description::belonging_to(&package)
            .load::<models::Description>(&self.conn)?;
        let version_texts: Vec<models::VersionText> = models::VersionText::belonging_to(&versions)
            .load::<models::VersionText>(&self.conn)?;
        let dep_descriptions: Vec<models::DependencyDescription> =
            models::DependencyDescription::belonging_to(&dependencies)
                .load::<models::DependencyDescription>(&self.conn)?;

        // whoever's reading this, I wish you to have fun deciphering that
        let version_models: Vec<version::Full> = {
            let grouped_dep_descriptions = dep_descriptions.grouped_by(&dependencies);
            let grouped_dependencies = dependencies
                .into_iter()
                .zip(grouped_dep_descriptions)
                .grouped_by(&versions);
            let grouped_contents = contents.grouped_by(&versions);
            let grouped_texts = version_texts.grouped_by(&versions);

            versions
                .into_iter()
                .zip(grouped_dependencies
                    .into_iter()
                    .zip(grouped_contents
                        .into_iter()
                        .zip(grouped_texts)))
                .map(|x| (x.0, (x.1).0, ((x.1).1).0, ((x.1).1).1))
                .into_iter()
                .map(|(ver, version_deps, version_contents, texts)| {
                    let len = texts.len();
                    let mut changes: Vec<Localized> = Vec::with_capacity(len);
                    let mut readmes: Vec<Localized> = Vec::with_capacity(len);

                    for x in texts {
                        changes.push(Localized {
                            language: x.language,
                            text: x.changes,
                        });

                        readmes.push(Localized {
                            language: x.language,
                            text: x.readme,
                        });
                    };

                    let dependencies = version_deps.into_iter().map(|(dep, desc)| dependency::Full {
                        package: dep.package,
                        spec: dep.spec,
                        dep_type: dep.dep_type,
                        description: match dep.dep_type {
                            models::types::DependencyType::Optional => {
                                Some(desc.into_iter().map(|x| Localized {
                                    language: x.language,
                                    text: x.description,
                                }).collect())
                            }
                            _ => None,
                        },
                    }).collect();

                    version::Full {
                        version: ver.version,
                        changes,
                        readme: readmes,
                        // TODO
                        url: "".to_string(),
                        dependencies,
                        contents: version_contents.into_iter().map(|x| ContentNode {
                            node_type: x.node_type,
                            path: x.path,
                        }).collect(),
                        created: ver.created,
                    }
                }).collect()
        };

        Ok(package::Full {
            name: package.name,
            description: descriptions.into_iter().map(|x| Localized {
                language: x.language,
                text: x.description,
            }).collect(),
            website: package.website,
            license: package.license,
            authors: package.authors,
            maintainers: maintainers.into_iter().map(|username| user::Short {
                username,
            }).collect(),
            versions: version_models,
            downloads: package.downloads,
            likes,
            created: package.created,
            updated: package.updated,
        })
    }
}

pub struct GetUser(pub String);

impl Message for GetUser {
    type Result = Result<user::Full, Error>;
}

impl Handler<GetUser> for DbExecutor {
    type Result = Result<user::Full, Error>;

    fn handle(&mut self, msg: GetUser, _: &mut Self::Context) -> Self::Result {
        let username = msg.0;

        let user = schema::users::table
            .filter(lower(schema::users::username).eq(username.to_lowercase()))
            .get_result::<models::User>(&self.conn)?;

        Ok(user::Full {
            username: user.username,
            group: user.group,
            registered: user.registered,
        })
    }
}

pub struct GetPackages {
    pub page: u32,
    pub limit: u32,
}

impl Message for GetPackages {
    type Result = Result<Vec<package::Short>, Error>;
}

impl Handler<GetPackages> for DbExecutor {
    type Result = Result<Vec<package::Short>, Error>;

    fn handle(&mut self, msg: GetPackages, _: &mut Self::Context) -> Self::Result {
        let offset = (msg.page - 1) * msg.limit;

        let packages: Vec<models::Package> = schema::packages::table
            .offset(offset.into())
            .limit(msg.limit.into())
            .order_by(schema::packages::created.desc())
            .load(&self.conn)?;
        let descriptions: Vec<models::Description> = models::Description::belonging_to(&packages)
            .load(&self.conn)?;

        let maintainer_models: Vec<models::Maintainer> = models::Maintainer::belonging_to(&packages)
            .load(&self.conn)?;
        let referenced_users: Vec<models::User> = schema::users::table
            .filter(
                schema::users::id.eq(
                    diesel::dsl::any(maintainer_models
                        .iter()
                        .map(|x| *x.id().0)
                        .collect::<Vec<_>>()
                    )
                )
            ).load::<models::User>(&self.conn)?;
        let referenced_users: HashMap<i32, String> = referenced_users
            .into_iter()
            .map(|x| (x.id, x.username))
            .collect();
        let maintainers = maintainer_models
            .grouped_by(&packages)
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .map(|model| referenced_users.get(&model.user).unwrap().clone())
                    .collect::<Vec<_>>()
            });

        let likes: Vec<models::GroupedLike> = schema::likes::table
                .select((schema::likes::package, diesel::dsl::sql::<BigInt>("count(*) as likes")))
                .filter(schema::likes::package.eq(diesel::dsl::any(
                    packages.iter().map(|x| &x.name).collect::<Vec<_>>()
                )))
                .group_by(schema::likes::package)
                .load(&self.conn)?;
        let likes: HashMap<String, i32> = likes
            .into_iter()
            .map(|x| (x.package, x.likes as i32))
            .collect();
        let versions: Vec<models::Version> = models::Version::belonging_to(&packages)
            .load(&self.conn)?;
        let dependencies: Vec<models::Dependency> = models::Dependency::belonging_to(&versions)
            .load(&self.conn)?;

        let grouped_dependencies = dependencies.into_iter().grouped_by(&versions);
        let grouped_versions = versions
            .into_iter()
            .zip(grouped_dependencies)
            .grouped_by(&packages)
            .into_iter()
            .zip(descriptions.into_iter().grouped_by(&packages));

        Ok(packages
            .into_iter()
            .map(|x| {
                let likes_num = *likes.get(&x.name).unwrap();
                (x, likes_num)
            })
            .zip(
                maintainers.zip(
                    grouped_versions
                )
            )
            .map(|((package, likes), (maintainers, (versions, descriptions)))| {
                package::Short {
                    name: package.name,
                    description: descriptions.into_iter().map(|x| Localized {
                        language: x.language,
                        text: x.description,
                    }).collect(),
                    maintainers: maintainers.into_iter().map(|x| user::Short {
                        username: x,
                    }).collect(),
                    versions: versions.into_iter().map(|(version, dependencies)| version::Short {
                        version: version.version,
                        // TODO
                        url: "".to_string(),
                        dependencies: dependencies.into_iter().map(|x| dependency::Short {
                            package: x.package,
                            spec: x.spec,
                        }).collect(),
                    }).collect(),
                    downloads: package.downloads,
                    likes,
                }
            }).collect())
    }
}

pub struct CreatePackage(pub package::Full);

impl Message for CreatePackage {
    type Result = Result<(), Error>;
}

impl Handler<CreatePackage> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: CreatePackage, _: &mut Self::Context) -> Self::Result {
        self.conn.transaction::<(), Error, _>(|| {
            let name = &msg.0.name;

            insert_into(schema::packages::table).values(&models::NewPackage {
                name: &name,
                website: &msg.0.website,
                license: &msg.0.license,
                authors: &msg.0.authors,
            }).execute(&self.conn)?;

            let versions: HashMap<String, i32> = insert_into(schema::versions::table)
                .values(&msg.0.versions.iter().map(|x| models::NewVersion {
                    package: &name,
                    version: &x.version,
                }).collect::<Vec<_>>())
                .returning((schema::versions::version, schema::versions::id))
                .get_results(&self.conn)?
                .into_iter()
                .collect();

            let get_version_id = move |version: &version::Full| {
                *versions.get(&version.version).unwrap()
            };

            let dependencies: Vec<((i32, String), i32)> = {
                let mut values: Vec<models::NewDependency> = Vec::new();

                for version in msg.0.versions.iter() {
                    let version_id = get_version_id(version);

                    for dep in version.dependencies.iter() {
                        values.push(models::NewDependency {
                            package: &name,
                            version: version_id,
                            spec: &dep.spec,
                            dep_type: dep.dep_type,
                        });
                    }
                }

                insert_into(schema::dependencies::table)
                    .values(&values)
                    .returning(((schema::dependencies::version, schema::dependencies::package),
                                schema::dependencies::id))
                    .get_results(&self.conn)?
            };

            {
                let mut values: Vec<models::NewDependencyDescription> = Vec::new();

                for version in msg.0.versions.iter() {
                    let version_id = get_version_id(version);

                    for dep in version.dependencies.iter() {
                        if let Some(desc) = dep.description.as_ref() {
                            let dep_id = dependencies
                                .iter()
                                .find(|x| (x.0).0 == version_id && &(x.0).1 == &dep.package)
                                .unwrap().1;

                            for text in desc.iter() {
                                values.push(models::NewDependencyDescription {
                                    dependency: dep_id,
                                    language: text.language,
                                    description: &text.text,
                                });
                            }
                        }
                    }
                }

                insert_into(schema::dependency_descriptions::table)
                    .values(&values)
                    .execute(&self.conn)?;
            }

            {
                let mut values: Vec<models::NewContentNode> = Vec::new();

                for version in msg.0.versions.iter() {
                    let version_id = get_version_id(version);

                    for node in version.contents.iter() {
                        values.push(models::NewContentNode {
                            version: version_id,
                            path: &node.path,
                            node_type: node.node_type,
                        });
                    }
                }

                insert_into(schema::contents::table)
                    .values(&values)
                    .execute(&self.conn)?;
            }

            {
                let mut values: Vec<models::NewVersionText> = Vec::new();

                for version in msg.0.versions.iter() {
                    let version_id = get_version_id(version);

                    let mut changes: HashMap<models::types::Language, &Localized> = HashMap::new();

                    for text in version.changes.iter() {
                        changes.insert(text.language, &text);
                    }

                    for readme in version.readme.iter() {
                        let changes_text = changes.get(&readme.language).unwrap();

                        values.push(models::NewVersionText {
                            version: version_id,
                            language: readme.language,
                            changes: &changes_text.text,
                            readme: &readme.text,
                        });
                    }
                }

                insert_into(schema::version_texts::table)
                    .values(&values)
                    .execute(&self.conn)?;
            }

            insert_into(schema::descriptions::table).values(&msg.0.description.iter().map(|desc| {
                models::NewDescription {
                    package: &name,
                    language: desc.language,
                    description: &desc.text,
                }
            }).collect::<Vec<_>>()).execute(&self.conn)?;

            let users: HashMap<String, i32> = schema::users::table
                .select((schema::users::username, schema::users::id))
                .filter(lower(schema::users::username).eq(diesel::dsl::any(
                    &msg.0.maintainers.iter().map(|x| x.username.to_lowercase()).collect::<Vec<_>>()
                )))
                .get_results::<(String, i32)>(&self.conn)?
                .into_iter()
                .map(|x| (x.0.to_lowercase(), x.1))
                .collect();

            insert_into(schema::maintainers::table).values(&msg.0.maintainers.iter().map(|x| {
                models::NewMaintainer {
                    user: *users.get(&x.username.to_lowercase()).unwrap(),
                    package: &name,
                }
            }).collect::<Vec<_>>()).execute(&self.conn)?;

            Ok(())
        })
    }
}

pub struct CreateUser {
    pub username: String,
    pub password: [u8; 64],
    pub salt: [u8; 64],
    pub group: models::types::UserGroup,
}

impl Message for CreateUser {
    type Result = Result<i32, Error>;
}

impl Handler<CreateUser> for DbExecutor {
    type Result = Result<i32, Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        Ok(insert_into(schema::users::table)
            .values(&models::NewUser {
                username: &msg.username,
                password: &msg.password,
                salt: &msg.salt,
                group: msg.group,
            })
            .returning(schema::users::id)
            .get_result(&self.conn)?)
    }
}