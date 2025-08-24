use std::fs::read_to_string;
use std::sync::OnceLock;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct PlacementCenterConfig {
    #[serde(default = "default_node_id")]
    pub node_id: u32,

    #[serde(default = "default_grpc_port")]
    pub grpc_port: usize,

    #[serde(default = "default_http_port")]
    pub http_port: u32,
    
}

fn default_node_id() -> u32 {
    1
}

fn default_grpc_port() -> usize {
    9982
}

fn default_http_port() -> u32 {
    3000
}

static PLACEMENT_CENTER_CONF: OnceLock<PlacementCenterConfig> = OnceLock::new();

pub fn init_placement_center_conf_by_path(config_path: &String) -> &'static PlacementCenterConfig {
    PLACEMENT_CENTER_CONF.get_or_init(|| {
        let content = read_to_string(config_path).expect("Failed to read config_util file");
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