use snap;
use std::convert::From;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error {
    /// path is not directory
    NotDir,
    /// buffer is not empty
    NotEmpty,
    /// no associated value for key found
    NotFound,
    /// std::io::Error
    IoErrorKind(io::ErrorKind),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoErrorKind(err.kind())
    }
}

const MIN_COMPRESS_SIZE: usize = 2_048;

/// Get raw file associated with key in read only mode
pub fn get_raw_file(dir: &Path, key: &str) -> Result<fs::File, Error> {
    if !dir.is_dir() {
        return Err(Error::NotDir);
    }

    match fs::File::open(dir.join(key)) {
        Ok(file) => Ok(file),
        Err(err) => Err(match err.kind() {
            io::ErrorKind::NotFound => Error::NotFound,
            _ => Error::from(err),
        }),
    }
}

/// Write value associated to key to disk in folder dir.
pub fn write(dir: &Path, key: &str, value: &[u8]) -> Result<(), Error> {
    if !dir.is_dir() {
        return Err(Error::NotDir);
    }

    let mut file = fs::File::create(dir.join(key))?;
    if value.len() >= MIN_COMPRESS_SIZE {
        file.write_all(&[1])?; // store compression info
        let mut wtr = snap::Writer::new(file);
        wtr.write_all(&value)?;
    } else {
        file.write_all(&[0])?; // store compression info
        file.write_all(&value)?;
    }

    Ok(())
}

/// Read value associated to key from disk in folder dir.
pub fn read(dir: &Path, key: &str) -> Result<Vec<u8>, Error> {
    let mut value: Vec<u8> = Vec::new();
    read_into(&dir, &key, &mut value)?;

    Ok(value)
}

/// Read value associated to key from disk in folder dir into value buffer.
pub fn read_into(dir: &Path, key: &str, mut buf: &mut Vec<u8>) -> Result<(), Error> {
    if !buf.is_empty() {
        return Err(Error::NotEmpty);
    }

    let mut file = get_raw_file(&dir, &key)?;
    let mut is_compressed = [0];
    file.read_exact(&mut is_compressed)?;
    if is_compressed[0] == 0 {
        file.read_to_end(&mut buf)?;
    } else {
        let mut rdr = snap::Reader::new(file);
        rdr.read_to_end(&mut buf)?;
    }

    Ok(())
}

/// Delete value associated to key from disk in folder dir.
pub fn delete(dir: &Path, key: &str) -> Result<(), Error> {
    // needed for consistent error handling
    get_raw_file(&dir, &key)?;

    fs::remove_file(dir.join(key))?;

    Ok(())
}
