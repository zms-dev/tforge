use crate::protocol::{TrackerRequest, TrackerResponse};
use anyhow::Result;

pub struct TrackerClient {
    announce_url: String,
    client: reqwest::Client,
}

impl TrackerClient {
    pub fn new(announce_url: String, client: Option<reqwest::Client>) -> Self {
        Self {
            announce_url,
            client: client.unwrap_or_else(reqwest::Client::new),
        }
    }
}

impl PartialEq for TrackerClient {
    fn eq(&self, other: &Self) -> bool {
        self.announce_url == other.announce_url
    }
}

impl TrackerClient {
    pub async fn announce(&self, request: TrackerRequest) -> Result<TrackerResponse> {
        let req = self.client.get(self.announce_url.clone()).query(&request);
        let response = req.send().await?;
        let json = response.json::<TrackerResponse>().await?;
        Ok(json)
    }
}
