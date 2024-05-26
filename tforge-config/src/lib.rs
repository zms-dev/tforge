use anyhow::{Context, Error, Result};
use serde::Deserialize;
use serde_with::{serde_as, Bytes};

#[serde_as]
#[derive(Deserialize, Debug, PartialEq)]
pub struct ClientConfig {
    #[serde_as(as = "Bytes")]
    pub peer_id: [u8; 20],
}

impl TryFrom<&str> for ClientConfig {
    type Error = Error;

    fn try_from(contents: &str) -> Result<Self> {
        toml::from_str(contents).context("parsing toml")
    }
}

impl TryFrom<String> for ClientConfig {
    type Error = Error;

    fn try_from(contents: String) -> Result<Self> {
        ClientConfig::try_from(contents.as_str())
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ServerConfig {}

impl TryFrom<&str> for ServerConfig {
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
            peer_id = "1234567890-abcedfghi"
        "#;
        let result = ClientConfig::try_from(contents);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            ClientConfig {
                peer_id: *b"1234567890-abcedfghi"
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

    #[test]
    fn test_try_from_server_config() {
        let contents = "";
        let result = ServerConfig::try_from(contents);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ServerConfig {});
    }
}
