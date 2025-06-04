mod args;
mod config;
mod log_conf;

use crate::args::ArgsParams;
use crate::config::PlacementCenterConfig;
use crate::config::init_placement_center_conf_by_path;
use crate::log_conf::init_placement_center_log;
use clap::Parser;
use log::{error, info, warn};

fn main() {
    let args_params = ArgsParams::parse();
    println!("conf will using {}!", args_params.conf);
    let conf: &PlacementCenterConfig = init_placement_center_conf_by_path(&args_params.conf);
    println!("the node is is {}!", conf.node_id);
    println!("the node port is {}!", conf.grpc_port);

    // 初始化日志
    init_placement_center_log();

    info!("Hello Info CC");

    warn!("Hello Warn BB");

    error!("Hello Error AA")
}
