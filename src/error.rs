error_chain! {
    foreign_links {
        Io(::std::io::Error);
        TomlDe(::toml::de::Error);
        DbConnection(::diesel::ConnectionError);
    }
}
