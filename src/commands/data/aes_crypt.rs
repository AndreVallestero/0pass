// aes_crypt.rs
// Module to encrypt/decrypt strings with aes256

use block_modes::{BlockMode, Cbc, InvalidKeyIvLength, BlockModeError};
use block_modes::block_padding::Pkcs7;
use aes::Aes256;
use hex_literal::hex;
use sha2::{Sha256, Digest};
use std::string::FromUtf8Error;

const INIT_VEC:[u8; 16] = hex!("cad92adf1411a549613971dd246eaa8f"); // PRNG

pub fn decrypt(password:&String, cipher_data:&Vec<u8>) -> Result<String, AesCryptError> {
    // Generate cipher key
    let mut hasher = Sha256::new();
    hasher.input(password);
    let crypt_key = hasher.result();
    
    // Initilize cipher
    type Aes256Cbc = Cbc<Aes256, Pkcs7>;
    let cipher = Aes256Cbc::new_var(&crypt_key[..], &INIT_VEC)?;

    // Decrypt cipher data
    let decrypt_data = cipher.decrypt_vec(cipher_data)?;
    Ok(String::from_utf8(decrypt_data)?)
}

pub fn encrypt(password:&String, plain_data:&String) -> Result<Vec<u8>, AesCryptError> {
    // Generate cipher key
    let mut hasher = Sha256::new();
    hasher.input(password);
    let crypt_key = hasher.result();
    
    // Initilize cipher
    type Aes256Cbc = Cbc<Aes256, Pkcs7>;
    let cipher = Aes256Cbc::new_var(&crypt_key[..], &INIT_VEC)?;

    // Encrypt cipher data
    let encrypt_data = cipher.encrypt_vec(plain_data.as_bytes());
    Ok(encrypt_data)
}

pub enum AesCryptError {
    CipherInitFailed(InvalidKeyIvLength),
    Utf8CastFailed(FromUtf8Error),
    DecryptFailed(BlockModeError)
}

impl From<InvalidKeyIvLength> for AesCryptError {
    fn from(error: InvalidKeyIvLength) -> Self {
        Self::CipherInitFailed(error)
    }
}

impl From<FromUtf8Error> for AesCryptError {
    fn from(error: FromUtf8Error) -> Self {
        Self::Utf8CastFailed(error)
    }
}

impl From<BlockModeError> for AesCryptError {
    fn from(error: BlockModeError) -> Self {
        Self::DecryptFailed(error)
    }
}