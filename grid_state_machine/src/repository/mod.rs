use crate::db::DatabaseState;

pub struct Repository {
    db: DatabaseState,
}

impl Repository {
    pub fn new(db: DatabaseState) -> Self {
        Repository { db }
    }

    pub fn add_transaction(&self, key: &i32, value: Vec<u8>) -> Result<(), String> {
        let slice_ref: &[u8] = value.as_slice();
        
        match DatabaseState::insert_key(&self.db, &key, slice_ref) {
            Ok(()) => {
                Ok(())
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                Err("Repository: Failed to add to db".to_string())
            }
        }
    }

    pub fn get_transaction(&self, key: &i32) -> Result<Vec<u8>, String> {
        match DatabaseState::read_key(&self.db, &key) {
            Ok(result) => {
                Ok(result)
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                Err("Repository: Key not found".to_string())
            }
        }
    }
}
