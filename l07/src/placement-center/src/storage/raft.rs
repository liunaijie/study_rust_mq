use std::collections::HashMap;
use std::sync::Arc;
use raft::eraftpb::{ConfState, HardState, Entry, SnapshotMetadata};
use raft::RaftState;
use raft::prelude::Snapshot;
use crate::storage::rocksdb::RocksDBEngine;

pub struct RaftMachineStorage {
    pub uncommit_index: HashMap<u64, i8>,
    pub snapshot_metadata: SnapshotMetadata,
    // RocksDB 引擎句柄, 用于与 RocksDB 交互
    rocksdb_engine_handler: Arc<RocksDBEngine>
}

impl RaftMachineStorage {
    pub fn new(rocksdb_engine_handler: Arc<RocksDBEngine>) -> Self {
        let mut rc = RaftMachineStorage {
            uncommit_index: HashMap::new(),
            snapshot_metadata: SnapshotMetadata::default(),
            rocksdb_engine_handler,
        };
        rc.uncommit_index = rc.uncommit_index();
        rc.snapshot_metadata = rc.create_snapshot_metadata();
        return rc;
    }
    
    pub fn hard_state(&self) -> HardState {
        return HardState::default();
    }
    
    pub fn conf_state(&self) -> ConfState {
        return ConfState::default();
    }
    
}


impl RaftMachineStorage {
    
    pub fn raft_state(&self) -> RaftState {
        // 启动时, 从存储中获取 Raft 状态
        // 这里返回一个新的 RaftState, 实际应用中应该从存储中读取,如果不存在则返回默认值
        RaftState {
            hard_state: HardState::new_(),
            conf_state: ConfState::new_(),
        }
    }
 
    pub fn first_index(&self) -> u64 {
        // 获取第一个索引, 实际应用中应该从存储中读取
        1
    }
    
    pub fn last_index(&self) -> u64 {
        // 获取最后一个索引, 实际应用中应该从存储中读取
        1000 // 示例值, 实际应用中应从存储中获取
    }
    
    pub fn entry_by_idx(&self, idx: u64) -> Option<Entry> {
    // 根据索引获取条目, 实际应用中应该从存储中读取
        // 如果不存在, 返回 None
        return None;
    }
    
    pub fn snapshot(&self) -> Snapshot {
        // 保存快照
        return Snapshot::new_();
    }
    
}


impl RaftMachineStorage {
    
    pub fn uncommit_index(&self) -> HashMap<u64, i8> {
        return HashMap::new();
    }
    
    pub fn create_snapshot_metadata(&self) -> SnapshotMetadata {
        // 
        let hard_state = self.hard_state();
        let conf_state = self.conf_state();
        
        let mut meta = SnapshotMetadata::default();
        meta.set_conf_state(conf_state);
        meta.set_index(self.last_index());
        meta.set_term(hard_state.term);
        return meta;
    }
    
}