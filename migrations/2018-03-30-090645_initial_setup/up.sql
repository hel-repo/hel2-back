CREATE TABLE users (
    id SERIAL CONSTRAINT users_id_pk PRIMARY KEY,
    username VARCHAR(32) NOT NULL,
    password CHAR(64) NOT NULL,
    salt CHAR(64) NOT NULL,
    "group" VARCHAR(16) CONSTRAINT users_group_values CHECK ("group" IN ('user', 'admin', 'banned')) NOT NULL
        DEFAULT 'user'
);

CREATE TABLE packages (
    name TEXT CONSTRAINT packages_name_pk PRIMARY KEY,
    website TEXT NOT NULL DEFAULT '',
    license TEXT NOT NULL DEFAULT '',
    authors TEXT[] NOT NULL DEFAULT '{}'
);

CREATE TABLE versions (
    id SERIAL CONSTRAINT versions_id_pk PRIMARY KEY,
    package TEXT CONSTRAINT versions_package_fk REFERENCES packages ON DELETE CASCADE NOT NULL,
    version TEXT NOT NULL
);

CREATE TABLE dependencies (
    id SERIAL CONSTRAINT dependencies_id_pk PRIMARY KEY,
    package TEXT CONSTRAINT dependencies_package_fk REFERENCES packages ON DELETE CASCADE NOT NULL,
    version INTEGER CONSTRAINT dependencies_version_fk REFERENCES versions ON DELETE CASCADE NOT NULL,
    spec TEXT NOT NULL DEFAULT '*',
    type VARCHAR(16) CONSTRAINT dependencies_type_values
        CHECK (type IN ('build-require', 'runtime-require', 'optional')) NOT NULL DEFAULT 'runtime-require'
);

CREATE TABLE contents (
    id SERIAL CONSTRAINT contents_id_pk PRIMARY KEY,
    version INTEGER CONSTRAINT contents_version_fk REFERENCES versions ON DELETE CASCADE NOT NULL,
    path TEXT NOT NULL,
    type VARCHAR(4) CONSTRAINT contents_type_values CHECK(type IN ('dir', 'file')) NOT NULL
);

CREATE TABLE maintainers (
    "user" INTEGER CONSTRAINT maintainers_user_fk REFERENCES users ON DELETE CASCADE,
    package TEXT CONSTRAINT maintains_package_fk REFERENCES packages ON DELETE CASCADE,
    CONSTRAINT maintainers_pk PRIMARY KEY ("user", package)
);

CREATE TABLE descriptions (
    package TEXT CONSTRAINT descriptions_package_fk REFERENCES packages ON DELETE CASCADE NOT NULL,
    "language" CHAR(2) CONSTRAINT descriptions_language_values
        CHECK ("language" IN ('ru', 'en')) NOT NULL,
    description TEXT NOT NULL,
    CONSTRAINT descriptions_pk PRIMARY KEY (package, "language")
);

CREATE TABLE version_texts (
    version INTEGER CONSTRAINT version_texts_version_fk REFERENCES versions ON DELETE CASCADE NOT NULL,
    "language" CHAR(2) CONSTRAINT version_texts_language_values
        CHECK ("language" IN ('ru', 'en')) NOT NULL,
    changes TEXT NOT NULL DEFAULT '',
    readme TEXT NOT NULL DEFAULT '',
    CONSTRAINT version_texts_pk PRIMARY KEY (version, "language")
);

CREATE TABLE dependency_descriptions (
    dependency INTEGER CONSTRAINT dependency_descriptions_dependency_fk REFERENCES dependencies ON DELETE CASCADE
        NOT NULL,
    "language" CHAR(2) CONSTRAINT dependency_descriptions_language_values
        CHECK ("language" IN ('ru', 'en')) NOT NULL,
    description TEXT NOT NULL,
    CONSTRAINT dependency_descriptions_pk PRIMARY KEY (dependency, "language")
);