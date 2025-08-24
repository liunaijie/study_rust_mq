use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct StorageDataWrap {
    pub data:Vec<u8>,
    pub create_time : u64
}

impl StorageDataWrap {
    pub fn new(data:Vec<u8>) -> Self {
        return StorageDataWrap {
            data,
            create_time: Self::now_second(),
        }
    }

    fn now_second() -> u64 {
        return SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
}

