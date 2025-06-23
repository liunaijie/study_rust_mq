pub mod logs;
pub mod args;
pub mod config;


use clap::Parser;
use log::{error, info, warn};
use tokio::signal;
use tokio::sync::broadcast;
use placement_center::server::grpc::grpc_server::start_grpc_server;
use placement_center::server::http::http_server::start_http_server;
use crate::args::ArgsParams;
use crate::config::init_placement_center_conf_by_path;
use crate::logs::log_init;

#[tokio::main]
async fn main() {

    let args_params = ArgsParams::parse();
    log_init(&args_params.log_config, &args_params.log_path);
    info!("Hello,World!");
    let placement_config = init_placement_center_conf_by_path(&args_params.conf);
    warn!("placement center node id is {}", placement_config.node_id);
    error!("placement center grpc port is {}", placement_config.grpc_port);
    let (stop_send, _) = broadcast::channel(2);
    start_server(placement_config.grpc_port,placement_config.http_port,stop_send).await;
}


pub async fn start_server(grpc_port:usize, http_port:u32,stop_sx: broadcast::Sender<bool>) {
    let raw_stop_sx = stop_sx.clone();
    tokio::spawn(async move {
        start_grpc_server(grpc_port,raw_stop_sx).await;
    });
    let raw_stop_sx = stop_sx.clone();
    tokio::spawn(async move {
        start_http_server(http_port,raw_stop_sx).await;
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