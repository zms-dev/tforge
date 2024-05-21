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
        info_hash: todo!(),
        peer_id: todo!(),
        port: todo!(),
        uploaded: todo!(),
        downloaded: todo!(),
        left: todo!(),
        compact: todo!(),
        event: todo!(),
        ip: todo!(),
        numwant: todo!(),
        key: todo!(),
        trackerid: todo!(),
    };

    let response = client.announce(request).await?;
    println!("{:?}", response);

    Ok(())
}
