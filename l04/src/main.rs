use clap::Parser;
use clap::builder::Str;
use log::{error, info, warn};
use log4rs;
use log4rs::config;
use serde::Deserialize;
use std::fs::{File, read_to_string as read_file};
use std::path::Path;
use std::sync::OnceLock;
use toml;

pub const DEFAULT_PLACEMENT_CENTER_CONFIG: &str = "config/placement-center.toml";

#[derive(Parser, Debug)]
#[command(next_line_help = true)]
struct ArgsParams {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = String::from(DEFAULT_PLACEMENT_CENTER_CONFIG))]
    conf: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct PlacementCenterConfig {
    #[serde(default = "default_node_id")]
    pub node_id: u32,

    #[serde(default = "default_grpc_port")]
    pub grpc_port: usize,

    pub log: Log,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Log {
    pub log_config: String,
    pub log_path: String,
}

pub fn default_node_id() -> u32 {
    1
}

pub fn default_grpc_port() -> usize {
    9982
}

static PLACEMENT_CENTER_CONF: OnceLock<PlacementCenterConfig> = OnceLock::new();

pub fn init_placement_center_conf_by_path(config_path: &String) -> &'static PlacementCenterConfig {
    PLACEMENT_CENTER_CONF.get_or_init(|| {
        let content = read_file(config_path).expect("Failed to read config file");
        let pc_config: PlacementCenterConfig = toml::from_str(&content).unwrap();
        return pc_config;
    })
}

pub fn placement_center_conf() -> &'static PlacementCenterConfig {
    match PLACEMENT_CENTER_CONF.get() {
        Some(config) => {
            return config;
        }
        None => {
            panic!(
                "Placement center configuration is not initialized, check the configuration file."
            )
        }
    }
}

fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

fn create_fold(path: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}

pub fn init_placement_center_log() {
    let conf = placement_center_conf();
    if !file_exists(&conf.log.log_config) {
        panic!(
            "Logging configuration file {} does not exist",
            conf.log.log_config
        )
    }

    match create_fold(&conf.log.log_path) {
        Ok(()) => {}
        Err(_e) => {
            panic!("Failed to initialize log directory {}", conf.log.log_path)
        }
    }

    let content = match read_file(&conf.log.log_config) {
        Ok(data) => data,
        Err(e) => {
            panic!("{}", e.to_string());
        }
    };

    let config_content = content.replace("{$path}", &conf.log.log_path);
    print!("using log config is \n{}", config_content);

    let config = match serde_yaml::from_str(&config_content) {
        Ok(data) => data,
        Err(e) => {
            panic!("{}", e.to_string())
        }
    };

    match log4rs::init_raw_config(config) {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

fn main() {
    let args = ArgsParams::parse();
    println!("conf will using {}!", args.conf);
    let conf: &PlacementCenterConfig = init_placement_center_conf_by_path(&args.conf);
    println!("the node is is {}!", conf.node_id);
    println!("the node port is {}!", conf.grpc_port);

    // 初始化日志
    init_placement_center_log();

    info!("Hello Info");

    warn!("Hello Warn");

    error!("Hello Error")
}
