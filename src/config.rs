use std::path::Path;
use serde::Deserialize;
use config_file::{ConfigFileError, FromConfigFile};
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct TestConfig {
    pub host: String,
    pub port: u64,
    pub tags: Vec<String>,
    pub inner: TestConfigInner,
}

#[derive(Debug, Deserialize)]
pub struct TestConfigInner {
    pub answer: u8,
}

pub fn read_config<P: AsRef<Path>>(p: P) -> Result<TestConfig, ConfigFileError> {
    let config = TestConfig::from_config_file(p)?;
    info!("{:#?}", config);
    Ok(config)
}

#[cfg(test)]
mod test {
   use super::*;
    #[test]
    fn test_read_config() {
        let cfg = read_config("config.toml").unwrap();
        println!("{:#?}", cfg);
    }
}