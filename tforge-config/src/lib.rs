use anyhow::{Context, Error, Result};
use serde::Deserialize;
use toml;

#[derive(Deserialize, Debug, PartialEq)]
pub struct ClientConfig {
    pub peer_id: String,
}

impl TryFrom<&str> for ClientConfig {
    type Error = Error;

    fn try_from(contents: &str) -> Result<Self> {
        toml::from_str(contents).context("parsing toml")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_client_config() {
        let contents = r#"
            peer_id = "test"
        "#;
        let result = ClientConfig::try_from(contents);
        assert!(!result.is_err());
        assert_eq!(
            result.unwrap(),
            ClientConfig {
                peer_id: "test".to_string()
            }
        );
    }

    #[test]
    fn test_try_from_client_config_invalid() {
        let contents = r#"
            peer_id = 1
        "#;
        let result = ClientConfig::try_from(contents);
        assert!(result.is_err());
    }

    #[test]
    fn test_try_from_client_config_empty() {
        let contents = "";
        let result = ClientConfig::try_from(contents);
        assert!(result.is_err());
    }
}
