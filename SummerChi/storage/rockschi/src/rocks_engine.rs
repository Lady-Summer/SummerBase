use rocksdb::DB;
use std::sync::Arc;

pub struct RocksEngine<'a> {

    inner_db: Arc<&'a DB>

}