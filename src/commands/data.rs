// data.rs
// Abstraction layer over writing and reading passbook

mod crossfile;
mod aes_crypt;
mod json_struct;

pub use json_struct::Entry;
pub use json_struct::Passbook;

use crossfile::*;
use aes_crypt::*;
use json_struct::*;

pub fn read_data(password:&String) -> Result<Passbook, DataError> {
    // Read file
    let mut enc_data = read_file()?;

    // Create a new file if empty
    if enc_data.len() == 0 {
        println!("Passbook not found or initialized, creating a new one");
        write_data(password, Passbook::default())?;

        // Re-reading
        enc_data = read_file()?;
    }

    // Decrypting
    let json_str = decrypt(password, &enc_data)?;

    // Deserializing, creating struct
    Ok(deser_json(json_str)?)
}

pub fn write_data(password:&String, passbook:Passbook) -> Result<(), DataError> {
    // Serialize struct to string
    let json_str = ser_json(passbook)?;
    
    // Encrypt string
    let enc_str = encrypt(password, &json_str)?;
    
    // Write file
    Ok(write_file(&enc_str)?)
}

pub enum DataError {
    ReadWriteFailed(std::io::Error),
    CryptFailed(AesCryptError),
    JsonFailed(serde_json::error::Error)
}

impl From<std::io::Error> for DataError {
    fn from(error: std::io::Error) -> Self {
        Self::ReadWriteFailed(error)
    }
}

impl From<AesCryptError> for DataError {
    fn from(error: AesCryptError) -> Self {
        Self::CryptFailed(error)
    }
}

impl From<serde_json::error::Error> for DataError {
    fn from(error: serde_json::error::Error) -> Self {
        Self::JsonFailed(error)
    }
}