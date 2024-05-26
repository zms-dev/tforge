use std::fmt::{Display, Write};

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    List(Vec<Value>),
    Named(String, Box<Value>),
    None(),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::List(l) => {
                write!(f, "[")?;
                for v in l {
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Named(n, v) => write!(f, "{}={}", n, v),
            Value::None() => Ok(()),
        }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(encode_chars(&s))
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(encode_chars(s))
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::String(if b {
            "true".to_string()
        } else {
            "false".to_string()
        })
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::String(i.to_string())
    }
}

impl From<u64> for Value {
    fn from(i: u64) -> Self {
        Value::String(i.to_string())
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::String(f.to_string())
    }
}

impl From<char> for Value {
    fn from(c: char) -> Self {
        Value::String(encode_chars(&c.to_string()))
    }
}

impl From<&[u8]> for Value {
    fn from(b: &[u8]) -> Self {
        Value::String(encode_bytes(b))
    }
}

impl From<Vec<Value>> for Value {
    fn from(l: Vec<Value>) -> Self {
        Value::List(l)
    }
}

fn encode_chars(chars: &str) -> String {
    let mut buff = [0; 4];
    chars
        .chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            _ => {
                c.encode_utf8(&mut buff);
                encode_bytes(&buff[0..c.len_utf8()])
            }
        })
        .collect::<String>()
}

fn encode_bytes(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "%{b:02X}");
        output
    })
}
