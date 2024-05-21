use anyhow::Result;
use std::path::PathBuf;

pub async fn main(config: &PathBuf) -> Result<()> {
    let contents = tokio::fs::read_to_string(config).await;
    Ok(())
}
