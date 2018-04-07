error_chain! {
    errors {
        UnknownGroup(group: String) {
            description("unknown group")
            display("unknown group: {}", group)
        }

        UnknownDependencyType(name: String) {
            description("unknown dependency type")
            display("unknown dependency type: {}", name)
        }

        UnknownNodeType(name: String) {
            description("unknown node type")
            display("unknown node type: {}", name)
        }

        UnknownLanguage(lang: String) {
            description("unknown language")
            display("unknown language: {}", lang)
        }
    }

    foreign_links {
        Io(::std::io::Error);
        TomlDe(::toml::de::Error);
        DbConnection(::diesel::ConnectionError);
    }
}

// Isn't there any better way than this?
unsafe impl Sync for Error {}