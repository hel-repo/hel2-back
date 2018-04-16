use actix::{Message, Handler};
use diesel::prelude::*;

use ::error::Result;
use ::models::*;
use super::DbExecutor;
use super::models;
use super::schema;

pub struct GetPackage(String);

impl Message for GetPackage {
    type Result = Result<package::Package>;
}

impl Handler<GetPackage> for DbExecutor {
    type Result = Result<package::Package>;

    fn handle(&mut self, msg: GetPackage, _: &mut Self::Context) -> Self::Result {
        let name: &String = &msg.0;

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

                    let dependencies = {
                        let mut dependencies: Vec<dependency::Full> = Vec::with_capacity(version_deps.len());

                        for item in version_deps {
                            let (dep, desc) = item;

                            dependencies.push(dependency::Full {
                                package: dep.package,
                                spec: dep.spec,
                                dep_type: dep.dep_type,
                                description: match dep.dep_type {
                                    models::types::DependencyType::Optional => {
                                        Some(desc.into_iter().map(|x| Localized {
                                            language: x.language,
                                            text: x.description,
                                        }).collect())
                                    },
                                    _ => None,
                                }
                            });
                        };

                        dependencies
                    };

                    version::Full {
                        version: ver.version,
                        changes,
                        readme: readmes,
                        // TODO
                        url: "".to_string(),
                        dependencies,
                        contents: {
                            let mut contents: Vec<ContentNode> = Vec::with_capacity(version_contents.len());

                            for x in version_contents {
                                contents.push(ContentNode {
                                    node_type: x.node_type,
                                    path: x.path,
                                });
                            }

                            contents
                        },
                        created: ver.created,
                    }
                })
                .collect()
        };

        Ok(package::Package::Full(package::Full {
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
        }))
    }
}