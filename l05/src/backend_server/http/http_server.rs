use std::net::SocketAddr;
use log::info;
use tokio::select;
use tokio::sync::broadcast;
use crate::backend_server::http::route::routes;

pub async fn start_http_server(stop_sx: broadcast::Sender<bool>) {
    let app = routes();

    let mut stop_rx = stop_sx.subscribe();
    // 添加端口监听,失败后抛出异常
    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
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


