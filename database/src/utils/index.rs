#![allow(unused)]

use SeekFrom::Start;
use std::io::{ErrorKind, Read, Seek, SeekFrom};
use crate::utils::database::Database;
use std::io::Error;

const U32_SIZE: usize = size_of::<u32>();

#[derive(Clone, Debug)]
pub struct Index {
    pub id: u32,
    pub offset: u32,
    pub length: u32,
    pub is_free: u8,
}

impl Index {
    pub fn new(
        content: String,
        collection_db: &mut Database,
        indexing_db: &mut Database,
    ) -> Result<Self, Error> {
        let (id, t_offset) = Self::find_free_space_for(content.len() as u32, indexing_db);
        let mut index = Self {
            length: content.len() as u32,
            is_free: 0,
            id,
            offset: 0,
        };
        let offset = match t_offset {
            None => {
                index.offset = collection_db.file.metadata()?.len() as u32;
                index.offset
            }
            Some(v) => {
                index.offset = v;
                v
            }
        };
        collection_db.write_at(offset, content.as_bytes().to_vec())?;
        index.save(indexing_db);
        Ok(index)
    }

    pub fn save(&mut self, db: &mut Database) -> Result<(), Error> {
        let bytes = self.clone().to_bytes();
        let offset = bytes.len() as u32 * self.id;
        db.write_at(offset, bytes)?;
        Ok(())
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.offset.to_le_bytes());
        bytes.extend_from_slice(&self.length.to_le_bytes());
        bytes.push(self.is_free);
        bytes
    }

    fn extract_u32(bytes: &mut Vec<u8>) -> u32 {
        let byte_slice: [u8; 4] = bytes
            .drain(0..U32_SIZE)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        u32::from_le_bytes(byte_slice)
    }

    pub fn take_first_from_bytes(bytes: &mut Vec<u8>) -> Self {
        let id = Self::extract_u32(bytes);
        let offset = Self::extract_u32(bytes);
        let length = Self::extract_u32(bytes);
        let is_free = bytes.remove(0);
        Self {
            id,
            offset,
            length,
            is_free,
        }
    }

    pub fn find_free_space_for(length: u32, indexing_db: &mut Database) -> (u32, Option<u32>) {
        let indexes = Self::get_all_indexing(indexing_db).unwrap();
        if indexes.len() == 0 {
            return (0, Some(0))
        };
        let mut free_offset: Option<u32> = None;
        let mut largest_id = 0;
        for i in indexes {
            if i.id > largest_id {
                largest_id = i.id;
            }
            if free_offset.is_some() {
                continue;
            }
            if i.is_free == 1 && i.length >= length {
                free_offset = Some(i.offset);
                return (i.id, free_offset);
            }
        }
        (largest_id + 1, free_offset)
    }

    pub fn get_all_indexing(db: &mut Database) -> Result<Vec<Index>, Error> {
        let mut bytes = db.get_all_bytes()?;
        let mut indexes = vec![];
        while bytes.len() > 0 {
            indexes.push(Self::take_first_from_bytes(&mut bytes));
        }
        Ok(indexes)
    }

    pub fn clear(collection_db: &mut Database, indexing_db: &mut Database) -> Result<(), Error> {
        collection_db.file.set_len(0)?;
        indexing_db.file.set_len(0)?;
        Ok(())
    }

    pub fn get_content(&self,collection_db: &mut Database) -> Result<String, Error> {
        collection_db.file.seek(Start(self.offset as u64))?;
        let mut bytes = vec![0; self.length as usize];
        collection_db.file.read_exact(&mut bytes);
        let content = String::from_utf8(bytes).unwrap();
        Ok(content)
    }

    pub fn delete_at(&self,indexing_db: &mut Database) -> Result<(), Error> {
        let offset = self.clone().to_bytes().len() as u32 * self.id;
        indexing_db.file.seek(Start(offset as u64));
        let bytes = Self {
            id: self.id,
            offset: self.offset,
            length: self.length,
            is_free: 1,
        }
        .to_bytes();
        indexing_db.write_at(offset, bytes)?;
        Ok(())
    }
    pub fn update_with(mut self, collection_db: &mut Database, index_db: &mut Database, content: String) -> Result<Self, Error> {
        if(content.len() as u32 > self.length) {
            self.delete_at(index_db);
            return Ok(Self::new(content, collection_db, index_db)?);
        };
        self.length = content.len() as u32;
        self.is_free = 0;
        collection_db.write_at(self.offset, content.into_bytes());
        self.save(index_db);
        Ok(self)
    }
}
