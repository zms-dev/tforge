use anyhow::Result;
use std::{
    io::{BufReader, Cursor},
    path::PathBuf,
};
use tforge_config::ClientConfig;
use tforge_metainfo::MetaInfo;
use tforge_tracker::{client::TrackerClient, protocol::TrackerRequest};

pub async fn main(config: &PathBuf, torrent: &PathBuf) -> Result<()> {
    let contents = tokio::fs::read_to_string(config).await;
    let config = ClientConfig::try_from(contents?)?;
    println!("{:?}", config);

    let file_content = tokio::fs::read(torrent).await?;
    let mut reader = BufReader::new(Cursor::new(file_content));
    let metainfo: MetaInfo = tforge_bencode::deserializer::from_reader(&mut reader)?;
    println!("{:?}", metainfo.announce);

    let client = TrackerClient::new(metainfo.announce, None);
    let request = TrackerRequest {
        info_hash: *b"1234567890-abcedfghi",
        peer_id: *b"1234567890-abcedfghi",
        port: 12345,
        uploaded: 0,
        downloaded: 0,
        left: 0,
        compact: false,
        event: None,
        ip: None,
        numwant: None,
        key: None,
        trackerid: None,
    };

    let response = client.announce(request).await?;
    println!("{:?}", response);

    Ok(())
}
