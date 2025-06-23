use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// 存储到RocksDB中的包装结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageDataWrap {
    pub data: Vec<u8>,
    pub create_time: u64,
}

impl StorageDataWrap {
    pub fn new<T: Serialize>(data: T) -> Self {
        let data = match serde_json::to_vec(&data) {
            Ok(data) => data,
            Err(e) => {
                panic!("Failed to serialize data: {}", e);
            }
        };
        return StorageDataWrap {
            data,
            create_time: Self::now_second(),
        };
    }

    fn now_second() -> u64 {
        return SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    // 获取存储的数据,将存储到RocksDB中的数据反序列化为指定类型
    pub fn get_data<T: DeserializeOwned>(&self) -> T {
        match serde_json::from_slice(&self.data) {
            Ok(data) => data,
            Err(e) => panic!("Failed to deserialize data: {}", e),
        }
    }
    
}
