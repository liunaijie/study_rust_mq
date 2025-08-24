use crate::storage::data_wrap::StorageDataWrap;
use crate::storage::rocksdb::RocksDBEngine;
use std::sync::Arc;

pub struct KvStorage {
    rocksdb_engine_handler: Arc<RocksDBEngine>,
}

impl KvStorage {
    pub fn new(rocksdb_engine_handler: Arc<RocksDBEngine>) -> Self {
        KvStorage {
            rocksdb_engine_handler,
        }
    }

}
