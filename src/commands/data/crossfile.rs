// crossfile.rs
// System-agnostically reads and writes the data file in the appropriate directory

use dirs::config_dir;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{Error, ErrorKind, prelude::*};

const DIR:&str = env!("CARGO_PKG_NAME");
const FILENAME:&str = concat!(env!("CARGO_PKG_NAME"), "-json.aes");

// Return option None or Some
pub fn read_file() -> std::io::Result<Vec<u8>> {
    // Get standard system config dir, convert to result
    let mut path = config_dir()
        .ok_or(Error::new(ErrorKind::Other, "missing directory"))?;
    
    // Create parent directories if they don't exist
    path.push(DIR);
    create_dir_all(&path)?;

    // Create the file if it doesn't exist, then open it
    path.push(FILENAME);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(&path)?;
    
    // Read file into string buffer
    let mut data_buffer = Vec::new();
    file.read_to_end(&mut data_buffer)?;
    Ok(data_buffer)
}

pub fn write_file(data:&Vec<u8>) -> std::io::Result<()> {
    // Get standard system config dir, convert to result
    let mut path = config_dir()
        .ok_or(Error::new(ErrorKind::Other, "missing directory"))?;
    
    // Create parent directories if they don't exist
    path.push(DIR);
    create_dir_all(&path)?;

    // Create the file if it doesn't exist, then open it
    path.push(FILENAME);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;

    // Write to file
    file.write_all(data)?;
    Ok(())
}