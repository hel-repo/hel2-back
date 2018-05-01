#[derive(Fail, Debug)]
pub enum ParseError {
    #[fail(display = "unknown group: {}", group)]
    UnknownGroup {
        group: String,
    },
    #[fail(display = "unknown dependency type: {}", name)]
    UnknownDependencyType {
        name: String,
    },
    #[fail(display = "unknown node type: {}", name)]
    UnknownNodeType {
        name: String,
    },
    #[fail(display = "unknown language: {}", language)]
    UnknownLanguage {
        language: String,
    },
}