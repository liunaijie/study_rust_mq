use crate::storage::data_wrap::StorageDataWrap;
use rocksdb::{ColumnFamily, DB, DBCompactionStyle, Error, Options, SliceTransform};
use std::path::Path;

pub const DB_COLUMN_FAMILY_CLUSTER: &str = "cluster";

fn column_family_list() -> Vec<String> {
    let mut list = Vec::new();
    list.push(DB_COLUMN_FAMILY_CLUSTER.to_string());
    return list;
}

pub struct RocksDBEngine {
    pub db: DB,
}

impl RocksDBEngine {
    pub fn new() -> Self {
        let opts = Self::build_rocksdb_options();
        let db_path = format!("{}/{}", "data_storage", "_rocksdb");
        if !Path::new(&db_path).exists() {
            DB::open(&opts, db_path.clone()).unwrap();
        }
        let cf_list = rocksdb::DB::list_cf(&opts, &db_path).unwrap();
        let mut instance = DB::open_cf(&opts, db_path.clone(), &cf_list).unwrap();

        for family in column_family_list().iter() {
            if cf_list.iter().find(|cf| cf == &family).is_none() {
                match instance.create_cf(&family, &opts) {
                    Ok(()) => {}
                    Err(e) => {
                        panic!("{}", e);
                    }
                }
            }
        }

        return RocksDBEngine { db: instance };
    }

    pub fn write(
        &self,
        cf: &ColumnFamily,
        key: &str,
        value: StorageDataWrap,
    ) -> Result<(), String> {
        let data = bincode::serialize(&value).unwrap();
        self.db
            .put_cf(cf, key, data)
            .map_err(|err| format!("Failed to put to ColumnFamily:{:?}", err))
    }

    pub fn read(&self, cf: &ColumnFamily, key: &str) -> Option<StorageDataWrap> {
        match self.db.get_cf(cf, key) {
            Ok(opt) => match opt {
                Some(found) => match bincode::deserialize::<StorageDataWrap>(&found) {
                    Ok(data) => Some(data),
                    Err(err) => panic!("{}", err),
                },
                None => None,
            },
            Err(err) => panic!("{}", err),
        }
    }

    pub fn delete(&self, cf: &ColumnFamily, key: &str) -> Result<(), String> {
        return Ok(self.db.delete_cf(cf, key)?);
    }

    pub fn exist(&self, cf: &ColumnFamily, key: &str) -> bool {
        self.db.key_may_exist_cf(cf, key)
    }

    pub fn cf_cluster(&self) -> &ColumnFamily {
        return self.db.cf_handle(&DB_COLUMN_FAMILY_CLUSTER).unwrap();
    }

    fn build_rocksdb_options() -> Options {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts.set_max_open_files(1000);
        opts.set_use_fsync(false);
        opts.set_bytes_per_sync(8388608);
        opts.optimize_for_point_lookup(1024);
        opts.set_table_cache_num_shard_bits(6);
        opts.set_max_write_buffer_number(32);
        opts.set_write_buffer_size(536870912);
        opts.set_target_file_size_base(1073741824);
        opts.set_min_write_buffer_number_to_merge(4);
        opts.set_level_zero_stop_writes_trigger(2000);
        opts.set_level_zero_slowdown_writes_trigger(0);
        opts.set_compaction_style(DBCompactionStyle::Universal);
        opts.set_disable_auto_compactions(true);
        let transform = SliceTransform::create_fixed_prefix(10);
        opts.set_prefix_extractor(transform);
        opts.set_memtable_prefix_bloom_ratio(0.2);
        return opts;
    }
}
