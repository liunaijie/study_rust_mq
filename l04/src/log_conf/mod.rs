use crate::config::placement_center_conf;
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Log {
    pub log_config: String,
    pub log_path: String,
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

    let content = match read_to_string(&conf.log.log_config) {
        Ok(data) => data,
        Err(e) => {
            panic!("{}", e.to_string());
        }
    };

    let config_content = content.replace("{$path}", &conf.log.log_path);
    print!("using log config_util is \n{}", config_content);

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

fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

fn create_fold(path: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}
