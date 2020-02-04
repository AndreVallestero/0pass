// json_struct.rs
// Module to do json-struct data conversion

use serde::{Deserialize, Serialize};
use serde_json::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub label: String,
    pub fields: Vec<String>,
}

impl Default for Entry {
    fn default() -> Entry {
        Entry {
            label: "".to_string(),
            fields: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Passbook {
    pub version: String,
    pub entries: Vec<Entry>
}

impl Default for Passbook {
    fn default() -> Passbook {
        Passbook {
            version: env!("CARGO_PKG_VERSION").to_string(),
            entries: Vec::new()
        }
    }
}

// deserializie json
pub fn deser_json(json_str:String) -> Result<Passbook, Error> {
    Ok(Passbook::from(
        serde_json::from_str(&json_str)?
    ))
}

// serialize json
pub fn ser_json(passbook:Passbook) -> Result<String, Error> {
    serde_json::to_string(&passbook)
}