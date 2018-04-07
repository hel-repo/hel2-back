table! {
    contents (id) {
        id -> Int4,
        version -> Int4,
        path -> Text,
        #[sql_name = "type"]
        type_ -> Varchar,
    }
}

table! {
    dependencies (id) {
        id -> Int4,
        package -> Text,
        version -> Int4,
        spec -> Text,
        #[sql_name = "type"]
        type_ -> Varchar,
    }
}

table! {
    dependency_descriptions (dependency, language) {
        dependency -> Int4,
        language -> Bpchar,
        description -> Text,
    }
}

table! {
    descriptions (package, language) {
        package -> Text,
        language -> Bpchar,
        description -> Text,
    }
}

table! {
    likes (user, package) {
        user -> Int4,
        package -> Text,
        time -> Timestamp,
    }
}

table! {
    maintainers (user, package) {
        user -> Int4,
        package -> Text,
    }
}

table! {
    packages (name) {
        name -> Text,
        website -> Text,
        license -> Text,
        authors -> Array<Text>,
        downloads -> Int4,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Bytea,
        salt -> Bytea,
        group -> Varchar,
        registered -> Timestamp,
    }
}

table! {
    versions (id) {
        id -> Int4,
        package -> Text,
        version -> Text,
        created -> Timestamp,
    }
}

table! {
    version_texts (version, language) {
        version -> Int4,
        language -> Bpchar,
        changes -> Text,
        readme -> Text,
    }
}

joinable!(contents -> versions (version));
joinable!(dependencies -> packages (package));
joinable!(dependencies -> versions (version));
joinable!(dependency_descriptions -> dependencies (dependency));
joinable!(descriptions -> packages (package));
joinable!(likes -> packages (package));
joinable!(likes -> users (user));
joinable!(maintainers -> packages (package));
joinable!(maintainers -> users (user));
joinable!(version_texts -> versions (version));
joinable!(versions -> packages (package));

allow_tables_to_appear_in_same_query!(
    contents,
    dependencies,
    dependency_descriptions,
    descriptions,
    likes,
    maintainers,
    packages,
    users,
    versions,
    version_texts,
);
