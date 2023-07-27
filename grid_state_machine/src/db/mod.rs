use leveldb::database::Database as GRID_DB;
use leveldb::kv::KV;
use leveldb::options::{Options, WriteOptions, ReadOptions};

use std::error::Error;
use std::path::Path;

pub struct DatabaseState {
    pub db_path_string: String,
    pub database: GRID_DB<i32>,
}

impl DatabaseState {
    pub fn init(db_path_string: String) -> Self { 
        let path = Path::new(&db_path_string);

        let mut options = Options::new();
        options.create_if_missing = true;

        let db = open_database(&path).unwrap();
        
        Self {
            db_path_string: db_path_string.to_string(),
            database: db,
        }
    }


    pub fn insert_key(&self, key: &i32, value: &[u8]) -> Result<(), Box<dyn Error>> {
        let write_options = WriteOptions::new();
        self.database.put(write_options, key, value)?;
        
        Ok(())
    }


    pub fn read_key(&self, key: &i32) -> Result<Vec<u8>, Box<dyn Error>> {
        let read_options: ReadOptions<'static, i32> = ReadOptions::new();
        let data: Option<Vec<u8>> = self.database.get(read_options, key)?;
        let result: Vec<u8> = data.ok_or("DB: Key not found")?;

        Ok(result)            
    }
}


fn open_database(path: &Path) -> Result<GRID_DB<i32>, Box<dyn Error>> { 
    let mut options = Options::new();
    options.create_if_missing = true;

    let db = GRID_DB::open(path, options)?;
    
    Ok(db)
}

#[cfg(test)]
mod tests {
    use crate::db::{DatabaseState};

    fn init_database(db_path: String) -> (DatabaseState, String) {
        let db_state = DatabaseState::init(db_path.clone());
        (db_state, db_path)
    }

    #[test]
    fn test_open_database() {
        let (db_state, db_path) = init_database("./test_db_open".to_string());
        drop(db_state);
        std::fs::remove_dir_all(db_path)
            .expect("Failed to remove test db directory.");
    }

    #[test]
    fn test_insert_and_read_key() {
        let (db_state, db_path) = init_database("./test_db_insert_and_read_key"
            .to_string());
 
        let key = 369;
        let value = b"Hello, Meow!";
        let value_bytes = value.to_vec();

        db_state.insert_key(&key, value).unwrap();

        let result = db_state.read_key(&key).unwrap();

        drop(db_state);
        std::fs::remove_dir_all(db_path)
            .expect("Failed to remove test db directory.");

        assert_eq!(result, value_bytes);
    }
}
