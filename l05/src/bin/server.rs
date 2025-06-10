use log::info;
use tokio::signal;
use tokio::sync::broadcast;
use l05_1::backend_server::grpc::grpc_server::start_grpc_server;
use l05_1::backend_server::http::http_server::start_http_server;

#[tokio::main]
async fn main() {
    let (stop_send, _) = broadcast::channel(2);
    start_server(stop_send).await;
}

pub async fn start_server(stop_sx: broadcast::Sender<bool>) {
    let raw_stop_sx = stop_sx.clone();
    tokio::spawn(async move {
        start_grpc_server(raw_stop_sx).await;
    });
    let raw_stop_sx = stop_sx.clone();
    tokio::spawn(async move {
        start_http_server(raw_stop_sx).await;
    });
    awaiting_stop(stop_sx.clone()).await;
}

pub async fn awaiting_stop(stop_sx: broadcast::Sender<bool>) {
    signal::ctrl_c().await.expect("failed to listen for event");
    match stop_sx.send(true) {
        Ok(_) => {
            info!("{}", "When ctrl + c is received, the service starts to stop")
        }
        Err(e) => {
            panic!("{}", e)
        }
    }
}