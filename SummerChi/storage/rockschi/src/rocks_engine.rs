use rocksdb::DB;
use std::sync::Arc;
use std::io::Result;
use super::prelude::Type;

pub struct RocksEngine {
    inner_db: Arc<DB>
}

impl RocksEngine {
    pub fn get(&self, key: String) -> Result<Option<dyn Type>> {
        match self.inner_db.get(key) {
            Ok(data) => {
                match data {
                    Some(stream) => Ok(Some(Type::from(stream))),
                    None => Ok(None)
                }
            }
            Err(e) => {}
        }
    }

    pub fn put(&self, key: &String, value: &dyn Type) -> Result<()> {
        match self.inner_db.put(key, value) {
            Ok(resp) => Ok(resp),
            Err(e) =>
        }
    }
}