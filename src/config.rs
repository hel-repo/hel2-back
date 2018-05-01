use std::io::Read;
use std::fs::File;
use std::path::Path;

use failure::Error;

pub mod config_groups {
    #[derive(Debug, Clone, Deserialize)]
    pub struct DbGroup {
        pub url: String,
        pub threads: usize,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct HttpGroup {
        pub pagination_limit: u32,
        pub threads: Option<usize>,
        pub bind_address: String,
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: config_groups::DbGroup,
    pub http: config_groups::HttpGroup,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
        let mut s = String::with_capacity(2048);
        let mut f = File::open(path)?;
        f.read_to_string(&mut s)?;
        Ok(::toml::from_str(&s)?)
    }
}