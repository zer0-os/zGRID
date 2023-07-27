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
                Err("Repository: Failed to add to db.".to_string())
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
                Err("Repository: Key not found.".to_string())
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::db::DatabaseState;
    use crate::repository::Repository;

    fn init_repository(db_path: String) -> (Repository, String) {
        let db_state: DatabaseState = DatabaseState::init(db_path.clone());
        let repository = Repository::new(db_state);
        (repository, db_path)
    }

    #[test]
    fn test_add_transaction() {
        let (repository, db_path) = init_repository("./test_db_repository_add_tx".to_string());
        
        let key = 100;
        let value_raw = b"text_value";
        let value_vec = value_raw.to_vec();

        let add_transaction_result = repository.add_transaction(&key, value_vec).unwrap();
        
        drop(repository);
        std::fs::remove_dir_all(db_path)
            .expect("Failed to remove db directory.");
        
        assert_eq!(add_transaction_result, ());
    }


    #[test]
    fn test_get_transaction() {
        let (repository, db_path) = init_repository("./test_db_repository_get_tx".to_string());

        let key = 369;
        let value_raw = b"three-six-nine";
        let value_vec = value_raw.to_vec();

        let _add_transaction_result = repository.add_transaction(&key, value_vec.clone());
        let get_transaction_result = repository.get_transaction(&key).unwrap();
    
        drop(repository);
        std::fs::remove_dir_all(db_path)
            .expect("Failed to remove db directory.");

        assert_eq!(get_transaction_result, value_vec);
    }
}
