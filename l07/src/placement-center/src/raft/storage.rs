use log::info;
use raft::{Error, GetEntriesContext, RaftState, Storage, StorageError};
use raft::eraftpb::{Entry, Snapshot};
use crate::storage::raft::RaftMachineStorage;

struct RaftRocksDBStorage {
    pub core: RaftMachineStorage
}


impl Storage for RaftRocksDBStorage {
    
    // 初始化Raft状态
    fn initial_state(&self) -> raft::Result<RaftState> {
        Ok(self.core.raft_state())
    }
    
    fn entries(&self, low: u64, high: u64, max_size: impl Into<Option<u64>>, context: GetEntriesContext) -> raft::Result<Vec<Entry>> {
        // 检查索引范围
        if low < self.core.first_index() {
            return Err(Error::Store(StorageError::Compacted))
        }
        if high > self.core.last_index() + 1 {
            panic!("index out of bound (last: {}, high: {}", self.core.last_index()+1, high);
        }
        // 从存储中依次取出这个范围内的条目
        let mut entry_list:Vec<Entry> = Vec::new();
        for idx in low..=high {
                let sret = self.core.entry_by_idx(idx);
            if sret == None {
                continue;
            }
            entry_list.push(sret.unwrap());
        }
         Ok(entry_list)
    }

    fn term(&self, idx: u64) -> raft::Result<u64> {
        if idx == self.core.snapshot_metadata.index {
            return Ok(self.core.snapshot_metadata.term);
        }
        if idx < self.core.first_index() {
            return Err(Error::Store(StorageError::Compacted))
        }
        if idx > self.core.last_index() {
            return Err(Error::Store(StorageError::Unavailable))
        }
        if let Some(value) = self.core.entry_by_idx(idx) {
            return Ok(value.term);
        }
        return Ok(self.core.snapshot_metadata.term);
        
    }

    fn first_index(&self) -> raft::Result<u64> {
        let fi = self.core.first_index();
        return Ok(fi);
    }

    fn last_index(&self) -> raft::Result<u64> {
        let li = self.core.last_index();
        return Ok(li);
    }

    fn snapshot(&self, request_index: u64, to: u64) -> raft::Result<Snapshot> {
        info!("Node {} requests snapshot data", to);

        let mut snap = self.core.snapshot();
        if snap.get_metadata().index < request_index {
            snap.mut_metadata().index = request_index;
        }
        
        return Ok(snap);
        
    }
}