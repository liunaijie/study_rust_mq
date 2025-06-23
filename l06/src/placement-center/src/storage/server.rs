use crate::storage::rocksdb::RocksDBEngine;
use std::sync::{Arc, OnceLock};

static ROCKSDB_ENGINE_HANDLER: OnceLock<Arc<RocksDBEngine>> = OnceLock::new();

fn init_handler() -> &'static Arc<RocksDBEngine> {
    ROCKSDB_ENGINE_HANDLER.get_or_init(|| {
        let rocksdb_engine_handler = Arc::new(RocksDBEngine::new());
        return rocksdb_engine_handler;
    })
}

pub fn get_rocksdb_handler() -> &'static Arc<RocksDBEngine> {
    ROCKSDB_ENGINE_HANDLER.get().unwrap_or_else(|| init_handler())
}
