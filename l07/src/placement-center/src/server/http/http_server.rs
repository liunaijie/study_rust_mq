use std::fmt::format;
use std::net::SocketAddr;
use std::sync::Arc;
use log::info;
use tokio::select;
use tokio::sync::broadcast;
use crate::server::http::route::routes;
use crate::storage::rocksdb::RocksDBEngine;

pub async fn start_http_server(port:u32, stop_sx: broadcast::Sender<bool>) {
    let rocksdb_engine_handler = Arc::new(RocksDBEngine::new());
    
    
    let app = routes(rocksdb_engine_handler);

    let mut stop_rx = stop_sx.subscribe();
    let addr = format!("0.0.0.0:{}", port);
    info!("http server staring at : {}", addr);
    // 添加端口监听,失败后抛出异常
    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(data) => data,
        Err(e) => {
            panic!("{}", e);
        }
    };
    
    select! {
        val = stop_rx.recv() => {
            match val {
                Ok(flag) => {
                    if flag {
                        info!("Http Server stopped successfully");
                    }
                } 
                Err(_) => {}
            }
        },
        val = axum::serve(listener, app.clone()) => {
            match val {
                Ok(()) => {},
                Err(e) => {
                    panic!("{}", e);
                }
            }
        }
    }


}


