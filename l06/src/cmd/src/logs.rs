use std::fs;
use std::path::Path;

pub fn log_init(config_path:&str, log_path:&str) {
    if !file_exists(config_path) {
        panic!("Logging configuration file {} does not exist", config_path);
    }
    
    // match create_fold(log_path) {
    //     Ok(_) => {}
    //     Err(e) => {
    //         panic!("Failed to initialize log directory {}", log_path)
    //     }
    // }

    let content = match fs::read_to_string(&config_path) {
        Ok(data) => {data}
        Err(e) => {
            panic!("Failed to read log conf file {}", config_path)
        }
    };

    // let log_config_content = content.replace("${path}", log_path);

    let config = match serde_yaml::from_str(&content) {
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