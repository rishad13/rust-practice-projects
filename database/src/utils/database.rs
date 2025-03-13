#![allow(unused)]
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Read, Write};

pub struct Database {
    pub file: File,
}


impl Database {
    pub fn clear_database(&mut self) -> Result<(), std::io::Error> {
        self.file.set_len(0)
    }
    pub fn len(&self) -> u64 {
        self.file.metadata().unwrap().len()
    }

    pub fn new(filename: &str) -> Result<Self, std::io::Error> {
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename) {
            Ok(file) => Ok(Self { file }),
            Err(e) => Err(e),
        }
    }

    pub fn get_all_bytes(&mut self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = vec![];
        self.file.seek(SeekFrom::Start(0))?;
        self.file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
    pub fn write_at(&mut self,offset: u32, bytes: Vec<u8>) -> Result<(), std::io::Error> {
        self.file.seek(SeekFrom::Start(offset as u64))?;
        self.file.write_all(&bytes)?;
        self.file.flush()?;
        Ok(())
    }
}