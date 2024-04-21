use serde::Deserialize;
use tforge_bencode::deserializer::Deserializer;

#[derive(Deserialize, Debug, PartialEq)]
struct Torrent {
    announce: String,
}

#[test]
fn test_deserialize() {
    let test_data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("testdata");
    let test_file = test_data_dir.join("ubuntu-23.10.1-desktop-amd64.iso.torrent");
    let file = std::fs::File::open(test_file).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut deserializer = Deserializer::from_buffer(&mut reader);
    let torrent: Torrent = Deserialize::deserialize(&mut deserializer).unwrap();
    assert_eq!(
        torrent,
        Torrent {
            announce: "https://torrent.ubuntu.com/announce".to_string()
        }
    );
}
