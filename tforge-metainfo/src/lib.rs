use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct MetaInfo {
    /// A dictionary that describes the file(s) of the torrent.
    pub info: Info,

    /// The announce URL of the tracker
    pub announce: String,

    /// The announce-list is a list tracker URLs.
    /// This is an extention to the official specification
    #[serde(rename = "announce-list")]
    pub announce_list: Option<Vec<Vec<String>>>,

    /// The creation time of the torrent
    #[serde(rename = "creation date", with = "optional_system_time")]
    pub creation_date: Option<std::time::SystemTime>,

    /// Free-form textual comments of the author
    pub comment: Option<String>,

    /// Name and version of the program used to create the .torrent
    #[serde(rename = "created by")]
    pub created_by: Option<String>,

    /// The string encoding format used to generate the pieces part of the info dictionary
    pub encoding: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Info {
    pub name: String,

    #[serde(rename = "piece length")]
    pub piece_length: usize,

    #[serde(with = "pieces_bytes")]
    pub pieces: Vec<[u8; 20]>,

    #[serde(flatten)]
    pub file_info: FileInfo,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum FileInfo {
    SingleFile { length: usize },
    MultiFile { files: Vec<File> },
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct File {
    pub length: usize,
    pub path: Vec<String>,
}

mod optional_system_time {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<std::time::SystemTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<u64>::deserialize(deserializer).map(|secs| {
            secs.map(|secs| std::time::UNIX_EPOCH + std::time::Duration::from_secs(secs))
        })
    }
}

mod pieces_bytes {
    use serde::{de::Visitor, Deserializer};

    struct PiecesVisitor;

    impl<'de> Visitor<'de> for PiecesVisitor {
        type Value = Vec<[u8; 20]>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a byte array")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let r: Vec<[u8; 20]> = v
                .chunks_exact(20)
                .map(|chunk| chunk.try_into().expect("invalid piece length"))
                .collect();
            Ok(r)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<[u8; 20]>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(PiecesVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tforge_bencode::deserializer::Deserializer;

    #[test]
    fn test_deserialize() {
        let test_data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("testdata");
        let test_file = test_data_dir.join("ubuntu-23.10.1-desktop-amd64.iso.torrent");
        let file = std::fs::File::open(test_file).unwrap();
        let mut reader = std::io::BufReader::new(file);
        let mut deserializer = Deserializer::from_buffer(&mut reader);
        let got: MetaInfo = serde::Deserialize::deserialize(&mut deserializer).unwrap();
        let expected = MetaInfo {
            announce: "https://torrent.ubuntu.com/announce".to_string(),
            announce_list: Some(vec![
                vec!["https://torrent.ubuntu.com/announce".to_string()],
                vec!["https://ipv6.torrent.ubuntu.com/announce".to_string()],
            ]),
            creation_date: Some(
                std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1697466120),
            ),
            comment: Some("Ubuntu CD releases.ubuntu.com".to_string()),
            created_by: Some("mktorrent 1.1".to_string()),
            encoding: None,
            info: Info {
                name: "ubuntu-23.10.1-desktop-amd64.iso".to_string(),
                piece_length: 262144,
                pieces: vec![],
                file_info: FileInfo::SingleFile { length: 5173995520 },
            },
        };
        assert_eq!(got.announce, expected.announce);
        assert_eq!(got.announce_list, expected.announce_list);
        assert_eq!(got.creation_date, expected.creation_date);
        assert_eq!(got.comment, expected.comment);
        assert_eq!(got.created_by, expected.created_by);
        assert_eq!(got.encoding, expected.encoding);
        assert_eq!(got.info.name, expected.info.name);
        assert_eq!(got.info.piece_length, expected.info.piece_length);
        assert!(!got.info.pieces.is_empty());
        assert_eq!(got.info.file_info, expected.info.file_info);
    }
}
