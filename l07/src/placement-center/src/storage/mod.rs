use crate::storage::data_wrap::StorageDataWrap;
use crate::storage::rocksdb::RocksDBEngine;
use serde::Serialize;
use std::sync::Arc;

pub mod data_wrap;
mod kv;
pub mod rocksdb;
pub mod raft;

fn engine_save<T>(
    rocksdb_engine_handler: Arc<RocksDBEngine>,
    key_name: &str,
    value: T,
) -> Result<(), String>
where
    T: Serialize,
{
    //
    let cf = rocksdb_engine_handler.cf_cluster();

    let content = match serde_json::to_vec(&value) {
        Ok(data) => data,
        Err(e) => return Err(e.to_string()),
    };

    let data = StorageDataWrap::new(content);

    match rocksdb_engine_handler.write(cf, &key_name, &data) {
        Ok(_) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e.to_string());
        }
    };
}

fn engine_get(
    rocksdb_engine_handler: Arc<RocksDBEngine>,
    key_name: &str,
) -> Result<Option<StorageDataWrap>, String> {
    let cf = rocksdb_engine_handler.cf_cluster();
    match rocksdb_engine_handler.read(cf, key_name) {
        Ok(Some(data)) => return Ok(Some(data)),
        Ok(None) => Ok(None),
        Err(e) => {
            return Err(e.to_string());
        }
    }
}


