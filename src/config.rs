use std::fs;
use std::io::Read;

use serde::Deserialize;

fn default_workers() -> usize {
    3
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_workers")]
    pub workers: usize,
    pub bind: String,
    pub youtube: Option<YouTube>,
    pub niconico: Option<NicoNico>,
}

#[derive(Deserialize)]
pub struct YouTube {
    pub user: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct NicoNico {
    pub user: String,
    pub password: String,
}

pub fn load(file: &str) -> Result<Config, std::io::Error> {
    let mut f = fs::File::open(file)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    let cfg: Config = toml::from_str(&content)?;
    Ok(cfg)
}
