use std::io::Read;
use std::fs::File;
use std::path::Path;

use error::Result;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub address: String,
    pub db: String,
    pub db_threads: usize,
    pub http_threads: Option<usize>
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Config> {
        let mut s = String::with_capacity(2048);
        let mut f = File::open(path)?;
        f.read_to_string(&mut s)?;
        Ok(::toml::from_str(&s)?)
    }
}
