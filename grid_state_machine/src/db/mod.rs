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
    pub fn init() -> Self {
        
        let db_path_string = "./grid_db";
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


pub fn open_database(path: &Path) -> Result<GRID_DB<i32>, Box<dyn Error>> { 
    let mut options = Options::new();
    options.create_if_missing = true;

    let db = GRID_DB::open(path, options)?;
    
    Ok(db)
}
