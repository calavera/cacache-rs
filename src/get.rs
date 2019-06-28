//! Functions for reading from cache.
use std::path::Path;

use ssri::{Algorithm, Integrity};

use crate::content::read::{self, Reader};
use crate::errors::Error;
use crate::index::{self, Entry};

pub struct Get {
    reader: Reader,
}

impl std::io::Read for Get {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl Get {
    pub fn check(self) -> Result<Algorithm, Error> {
        self.reader.check()
    }
}

pub fn open<P, K>(cache: P, key: K) -> Result<Get, Error>
where
    P: AsRef<Path>,
    K: AsRef<str>,
{
    if let Some(entry) = index::find(cache.as_ref(), key.as_ref())? {
        let reader = read::open(cache.as_ref(), entry.integrity)?;
        Ok(Get { reader })
    } else {
        Err(Error::NotFound)
    }
}

pub fn open_hash<P>(cache: P, sri: Integrity) -> Result<Get, Error>
where
    P: AsRef<Path>
{
    Ok(Get {
        reader: read::open(cache.as_ref(), sri)?,
    })
}

pub fn read<P, K>(cache: P, key: K) -> Result<Vec<u8>, Error>
where
    P: AsRef<Path>,
    K: AsRef<str>,
{
    if let Some(entry) = index::find(cache.as_ref(), key.as_ref())? {
        read_hash(cache, &entry.integrity)
    } else {
        Err(Error::NotFound)
    }
}

pub fn read_hash<P>(cache: P, sri: &Integrity) -> Result<Vec<u8>, Error>
where
    P: AsRef<Path>
{
    Ok(read::read(cache.as_ref(), sri)?)
}

pub fn copy<P, K, Q>(cache: P, key: K, to: Q) -> Result<u64, Error>
where
    P: AsRef<Path>,
    K: AsRef<str>,
    Q: AsRef<Path>,
{
    if let Some(entry) = index::find(cache.as_ref(), key.as_ref())? {
        copy_hash(cache, &entry.integrity, to)
    } else {
        Err(Error::NotFound)
    }
}

pub fn copy_hash<P, Q>(cache: P, sri: &Integrity, to: Q) -> Result<u64, Error>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    read::copy(cache.as_ref(), sri, to.as_ref())
}

pub fn info<P, K>(cache: P, key: K) -> Result<Option<Entry>, Error>
where
    P: AsRef<Path>,
    K: AsRef<str>,
{
    index::find(cache.as_ref(), key.as_ref())
}

pub fn hash_exists<P: AsRef<Path>>(cache: P, sri: &Integrity) -> bool {
    read::has_content(cache.as_ref(), &sri).is_some()
}
