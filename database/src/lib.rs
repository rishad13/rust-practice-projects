mod utils;


#[cfg(test)]
mod tests {
    use crate::utils::database::Database;
    use crate::utils::index::Index;

    #[test]
    fn serialization_des() {
        let index = Index {
            id: 0,
            offset: 0,
            length: 5,
            is_free: 0
        };
        let index2 = Index::take_first_from_bytes(index.clone().to_bytes().as_mut());
        assert_eq!(index.id, index2.id);
        assert_eq!(index.offset, index2.offset);
        assert_eq!(index.length, index2.length);
        assert_eq!(index.is_free, index2.is_free);
    }

    #[test]
    fn save() {
        // Initialize databases for collections and index
        let mut collection_db = Database::new("collections").unwrap();
        let mut index_db = Database::new("index").unwrap();

        // Clear any existing data in both databases
        Index::clear(&mut collection_db, &mut index_db).unwrap();

        // Create first content string
        let content = "Hey works".to_string();

        // Index the first content - this should create index ID 0
        Index::new(content.clone(), &mut collection_db, &mut index_db).unwrap();

        // Create second content string with identical text
        let content2 = "Hey works".to_string();

        // Index the second content - this should create index ID 1
        Index::new(content2.clone(), &mut collection_db, &mut index_db).unwrap();

        // Retrieve all indexes from the database
        let mut indexes: Vec<Index> = Index::get_all_indexing(&mut index_db).unwrap();

        // Verify that two distinct indexes were created, despite identical content
        assert_eq!(indexes.len(), 2);

        // Verify first index properties
        let first_index: &mut Index = indexes.first_mut().unwrap();
        assert_eq!(first_index.id, 0);           // First index should have ID 0
        assert_eq!(first_index.offset, 0);       // First index starts at offset 0
        assert_eq!(first_index.length, content.len() as u32);  // Length should match content length

        let first_content = first_index.get_content(&mut collection_db).unwrap();
        assert_eq!(first_content, content); //read from db with index as same as the input (content)

        let update_content = "update with longer content Works".to_string();
        let updated_index = first_index.clone().update_with(&mut collection_db, &mut index_db, update_content.clone()).unwrap();
        let first_update_content = updated_index.get_content(&mut collection_db).unwrap();
        assert_eq!(update_content, first_update_content);

        // Verify second index properties
        let second_index = indexes.last().unwrap();
        assert_eq!(second_index.id, 1);          // Second index should have ID 1
        // Second index should start at offset equal to the length of the first content
        assert_eq!(second_index.offset, content.len() as u32);
        // Second index length should match second content length
        assert_eq!(second_index.length, content2.len() as u32);

        let second_content = second_index.get_content(&mut collection_db).unwrap();
        assert_eq!(second_content, content2); //read from db with index as same as the input (content)

        //check new content with less length save in the free space, since when updated the first index with larger content, that index (id) would be free
        let new_index = Index::new("small".to_string(), &mut collection_db, &mut index_db).unwrap();
        assert_eq!(new_index.id, 0);

        //since updating the previous index as non free, this should save last, id = 3
        let new_index_2 = Index::new("small".to_string(), &mut collection_db, &mut index_db).unwrap();
        assert_eq!(new_index_2.id, 3);
    }
}
