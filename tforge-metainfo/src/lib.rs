use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MetaInfo {
    /// NOTE: All fields must be sorted alphabetically!

    /// The announce URL of the tracker
    pub announce: String,

    /// The announce-list is a list tracker URLs.
    /// This is an extention to the official specification
    #[serde(rename = "announce-list")]
    pub announce_list: Option<Vec<Vec<String>>>,

    /// Free-form textual comments of the author
    pub comment: Option<String>,

    /// Name and version of the program used to create the .torrent
    #[serde(rename = "created by")]
    pub created_by: Option<String>,

    /// The creation time of the torrent
    #[serde(rename = "creation date", with = "optional_system_time")]
    pub creation_date: Option<std::time::SystemTime>,

    /// The string encoding format used to generate the pieces part of the info dictionary
    pub encoding: Option<String>,

    /// A dictionary that describes the file(s) of the torrent.
    pub info: Info,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Info {
    /// NOTE: All fields must be sorted alphabetically!

    #[serde(flatten)]
    pub file_info: FileInfo,

    pub name: String,

    #[serde(rename = "piece length")]
    pub piece_length: usize,

    #[serde(with = "pieces_bytes")]
    pub pieces: Vec<[u8; 20]>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum FileInfo {
    /// NOTE: All fields must be sorted alphabetically!
    SingleFile {
        length: usize,
    },
    MultiFile {
        files: Vec<File>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct File {
    pub length: usize,
    pub path: Vec<String>,
}

mod optional_system_time {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<std::time::SystemTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<u64>::deserialize(deserializer).map(|secs| {
            secs.map(|secs| std::time::UNIX_EPOCH + std::time::Duration::from_secs(secs))
        })
    }

    pub fn serialize<S>(
        data: &Option<std::time::SystemTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        data.map(|time| {
            time.duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        })
        .serialize(serializer)
    }
}

mod pieces_bytes {
    use serde::{de::Visitor, Deserializer, Serializer};

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

    pub fn serialize<S>(data: &Vec<[u8; 20]>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&data.concat())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{BufReader, Cursor},
        path::Path,
    };
    use tforge_bencode::{deserializer::from_reader, serializer::from_writer};

    #[test]
    fn test_bencode_real_torrent_file() {
        let test_data_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("testdata");
        let test_file = test_data_dir.join("ubuntu-23.10.1-desktop-amd64.iso.torrent");
        let file_content = std::fs::read(&test_file).unwrap();

        let mut reader = BufReader::new(Cursor::new(&file_content));
        let meta_info: MetaInfo = from_reader(&mut reader).unwrap();

        let mut buffer = Vec::new();
        let mut writer = from_writer(&mut buffer);
        meta_info.serialize(&mut writer).unwrap();

        assert_eq!(&file_content, &buffer);
    }

    #[test]
    fn test_bencode_single_file() {
        let meta_info = MetaInfo {
            announce: "http://example.com/announce".to_string(),
            announce_list: Some(vec![vec!["http://example.com/announce".to_string()]]),
            comment: Some("comment".to_string()),
            created_by: Some("created_by".to_string()),
            creation_date: Some(
                std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(123),
            ),
            encoding: Some("UTF-8".to_string()),
            info: Info {
                file_info: FileInfo::SingleFile { length: 123 },
                name: "name".to_string(),
                piece_length: 123,
                pieces: vec![
                    b"aaaaaaaaaaaaaaaaaaaa".to_owned(),
                    b"bbbbbbbbbbbbbbbbbbbb".to_owned(),
                ],
            },
        };

        let mut buffer = Vec::new();
        let mut writer = from_writer(&mut buffer);
        meta_info.serialize(&mut writer).unwrap();

        let mut reader = BufReader::new(Cursor::new(&buffer));
        let decoded_meta_info: MetaInfo = from_reader(&mut reader).unwrap();

        assert_eq!(meta_info, decoded_meta_info);
    }

    #[test]
    fn test_bencode_multi_file() {
        let meta_info = MetaInfo {
            announce: "http://example.com/announce".to_string(),
            announce_list: Some(vec![vec!["http://example.com/announce".to_string()]]),
            comment: Some("comment".to_string()),
            created_by: Some("created_by".to_string()),
            creation_date: Some(
                std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(123),
            ),
            encoding: Some("UTF-8".to_string()),
            info: Info {
                file_info: FileInfo::MultiFile {
                    files: vec![File {
                        length: 123,
                        path: vec!["path".to_string()],
                    }],
                },
                name: "name".to_string(),
                piece_length: 123,
                pieces: vec![
                    b"aaaaaaaaaaaaaaaaaaaa".to_owned(),
                    b"bbbbbbbbbbbbbbbbbbbb".to_owned(),
                ],
            },
        };

        let mut buffer = Vec::new();
        let mut writer = from_writer(&mut buffer);
        meta_info.serialize(&mut writer).unwrap();

        let mut reader = BufReader::new(Cursor::new(&buffer));
        let decoded_meta_info: MetaInfo = from_reader(&mut reader).unwrap();

        assert_eq!(meta_info, decoded_meta_info);
    }
}
